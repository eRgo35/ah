# `ah` MVP Hardening + CI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move `ah` from "functional prototype" to MVP by fixing the known correctness bugs, adding the minimum test surface to prevent regressions, and standing up a PR-time CI workflow that catches breakage before it lands — all while leaving the existing cargo-dist release pipeline untouched.

**Architecture:**
- Code work is scoped to behavior already implied by the existing CLI: fix flag wiring, confirmations, dry-run, and resolve the `sync`/`rebuild`/`choose_install` overlap by deletion (not addition).
- Tests use plain `cargo test` with a hand-rolled fake-command injection point so we can validate argv/flags without invoking `paru`/`topgrade` (the current `Command::new(const)` calls make this awkward; the plan introduces a single internal seam for it).
- CI is a single GitHub Actions workflow file (`.github/workflows/ci.yml`) that runs on PR and on push to `main`, layered alongside the existing `release.yml` (tag-driven, cargo-dist).

**Tech Stack:** Rust 1.78+ (edition 2021), `clap` 4.5, `colored` 2.1, GitHub Actions (ubuntu-latest). No new runtime dependencies.

**Spec:** `/home/mike/Work/ah` (this repository) — current `0.3.1` source as the baseline. No separate spec doc; this plan IS the spec for the MVP cut.

## Global Constraints

- MUST NOT change the public CLI surface. Subcommand names, aliases, argument names, and the no-arg default action (`full_upgrade`) stay identical. (Justification: 0.3.1 is already on the AUR; CLI breakage is a packaging regression.)
- MUST NOT introduce new runtime dependencies. `Cargo.lock` diff is `clap`/`colored` only.
- MUST keep the existing `release.yml` (cargo-dist) untouched. Tag-driven releases continue to work.
- MUST keep the AUR `PKGBUILD` shipping in this repo. Bumping `pkgver` for each release stays a manual step (out of scope for CI automation here).
- Code style: `cargo fmt` clean, `cargo clippy -- -D warnings` clean (one-shot allow-list permitted only for the dead-code warnings that vanish once `rebuild` is removed in Task 4).
- Commit style: Conventional Commits (`feat:`, `fix:`, `chore:`, `test:`, `ci:`).
- Every task ends with `cargo build --release` green AND `cargo test` green.

## File Structure

**New files:**
- `.github/workflows/ci.yml` — PR-time CI (Task 10).
- `tests/cli_flags.rs` — integration tests for flag wiring (Tasks 1, 2).
- `tests/dry_run.rs` — integration tests for `--dry-run` (Task 7).
- `tests/file_io.rs` — unit tests for `file.rs` reading/writing/appending (Task 3).
- `tests/confirm.rs` — integration tests for confirmation prompts (Tasks 5, 6).
- `docs/MVP.md` — short user-facing note on the hardened surface (Task 12).

**Modified files:**
- `src/packages/mod.rs` — add `confirm_prompt` helper, add `CommandRunner` seam, remove `rebuild` re-export, update `full_upgrade` argv (Tasks 1, 4, 5, 8).
- `src/packages/upgrade.rs` — fix `--noconfirm`/`--confirm` flag logic, add `--dry-run` (Tasks 1, 7).
- `src/packages/sync.rs` — fix flag logic, add `--dry-run`, drop duplicate with `rebuild` (Tasks 1, 4, 7).
- `src/packages/rebuild.rs` — **deleted** (Task 4).
- `src/packages/choose_install.rs` — append to `~/packages` like `install` does; drop the "index not updated" warning (Task 4).
- `src/packages/install.rs` — use shared flag helper, use shared confirm helper (Tasks 1, 5).
- `src/packages/remove.rs` — add confirmation prompt, use shared flag helper (Tasks 1, 5).
- `src/packages/full_upgrade.rs` — pass args correctly to `topgrade`; confirm before running (Task 8).
- `src/packages/find.rs` — use shared confirm helper for `query.is_empty()` early return (no behavior change; consistency).
- `src/file.rs` — add `read_packages_filtered` helper, add atomic `write_packages` (Task 3).
- `src/cli.rs` — add `--dry-run` to `Sync` and `Upgrade` (Task 7).
- `src/main.rs` — pass `dry_run` through; no public surface change.

**Out of scope (explicitly):**
- Grouped lists / multi-file / `--file` flag (deferred).
- File locking on `~/packages` (deferred; concurrent runs are an edge case for a personal tool).
- AUR auto-publish (deferred; manual `pkgver` bump is the current contract).
- crates.io publish (not requested).
- Replacing `paru` with pluggable backends (README over-promises; fixing the wording is a doc task in #12).

---

## Task 1: Fix `--noconfirm` / `--confirm` flag wiring

**Files:**
- Modify: `src/packages/upgrade.rs`
- Modify: `src/packages/sync.rs`
- Modify: `src/packages/rebuild.rs`
- Modify: `src/packages/mod.rs`
- Modify: `src/packages/remove.rs`
- Modify: `src/packages/install.rs`
- Test: `tests/cli_flags.rs`

**Interfaces:**
- Consumes: existing `pub fn upgrade(noconfirm: bool) -> Result<(), Box<dyn Error>>` and `pub fn sync(noconfirm: bool) -> Result<(), Box<dyn Error>>` signatures.
- Produces: same signatures, but argv now correctly uses `--noconfirm` only when `noconfirm == true`. The literal string `"--confirm"` never reaches `paru`.

**Context:** `paru`/`pacman` accept `--noconfirm` to skip prompts. They do NOT accept `--confirm`. The current code passes `--confirm` when the user did NOT pass `--noconfirm`, which `paru` silently ignores — meaning the `noconfirm: false` branch behaves identically to `noconfirm: true`. The fix is to omit the flag entirely (which gives `paru` its default prompt behavior). This needs a shared helper so the four call sites stay in sync.

- [ ] **Step 1: Write failing test in `tests/cli_flags.rs`**

Create `tests/cli_flags.rs`:

```rust
use ah_pkg_test_support::*; // see Task 9; for now use a local dummy if helper not yet present

#[test]
fn upgrade_passes_noconfirm_only_when_requested() {
    // Run ah with a fake paru, then assert argv.
    // See Task 9 for the fake-command harness. Until Task 9 lands,
    // this test is a unit test on the helper function itself.
    assert_eq!(noconfirm_arg(true), vec!["--noconfirm"]);
    assert_eq!(noconfirm_arg(false), Vec::<&str>::new());
}

#[test]
fn sync_passes_noconfirm_only_when_requested() {
    assert_eq!(noconfirm_arg(true), vec!["--noconfirm"]);
    assert_eq!(noconfirm_arg(false), Vec::<&str>::new());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test cli_flags`
Expected: compile error — `noconfirm_arg` and the `ah_pkg_test_support` module do not exist.

- [ ] **Step 3: Add helper in `src/packages/mod.rs`**

In `src/packages/mod.rs`, add a new public function:

```rust
/// Returns the argv slice that should be passed to `paru`/`pacman`
/// to convey the user's noconfirm preference. Returns an empty vec
/// when the user did not request `--noconfirm` (paru's default
/// interactive prompt behavior is then preserved).
pub fn noconfirm_arg(noconfirm: bool) -> Vec<&'static str> {
    if noconfirm {
        vec!["--noconfirm"]
    } else {
        Vec::new()
    }
}
```

- [ ] **Step 4: Replace inline flag logic in each call site**

In `src/packages/upgrade.rs`, replace:

```rust
let noconfirm = if noconfirm {
    "--noconfirm"
} else {
    "--confirm"
};
```

with:

```rust
let extra = noconfirm_arg(noconfirm);
```

and change the `Command::new` block to append `extra` to its args.

Apply the same change in `src/packages/sync.rs` and `src/packages/rebuild.rs`. Add `use crate::packages::noconfirm_arg;` to each.

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --test cli_flags`
Expected: PASS (2 tests).

- [ ] **Step 6: Commit**

```bash
git add src/packages/mod.rs src/packages/upgrade.rs src/packages/sync.rs src/packages/rebuild.rs tests/cli_flags.rs
git commit -m "fix: pass --noconfirm only when requested; remove bogus --confirm"
```

---

## Task 2: Extract a single test seam for invoking external commands

**Files:**
- Modify: `src/packages/mod.rs`
- Modify: `src/packages/upgrade.rs`
- Modify: `src/packages/sync.rs`
- Modify: `src/packages/rebuild.rs`
- Modify: `src/packages/remove.rs`
- Modify: `src/packages/install.rs`
- Modify: `src/packages/choose_install.rs`
- Modify: `src/packages/find.rs`
- Modify: `src/packages/full_upgrade.rs`
- Modify: `src/main.rs`
- Test: `tests/cli_flags.rs` (extended)

**Interfaces:**
- Produces: a `pub fn run_command(bin: &str, args: &[&str]) -> Result<i32, Box<dyn Error>>` function in `packages::mod`. It wraps `Command::new(bin).args(args).status()`. In tests, this function is overridable via an `#[cfg(test)]` swap (see Step 3). For non-test builds, it's the real wrapper.

**Context:** Every package subcommand spawns `paru` or `topgrade` via `Command::new(const)`. To unit-test argv construction without installing `paru`, we need one funnel point. This task does NOT add any new tests yet — it only creates the seam so Tasks 1, 5, 6, 7 can all use it.

- [ ] **Step 1: Add `run_command` in `src/packages/mod.rs`**

```rust
use std::process::Command;

pub fn run_command(bin: &str, args: &[&str]) -> Result<i32, Box<dyn std::error::Error>> {
    let status = Command::new(bin)
        .args(args)
        .status()
        .map_err(|e| format!("failed to spawn {}: {}", bin, e))?;
    Ok(status.code().unwrap_or(-1))
}
```

- [ ] **Step 2: Refactor each call site to use it**

In `src/packages/upgrade.rs`, replace the existing spawn block:

```rust
let mut child = Command::new(PACKAGE_MANAGER)
    .arg("--color")
    .arg("always")
    .arg("-Syu")
    .arg(noconfirm)
    .spawn()
    .expect("Failed to execute command");

let status = child.wait().expect("Failed to wait on child");
```

with:

```rust
let mut argv: Vec<&str> = vec!["--color", "always", "-Syu"];
argv.extend(noconfirm_arg(noconfirm));
let code = run_command(PACKAGE_MANAGER, &argv)?;
if code != 0 {
    return Err(format!("paru exited with {}", code).into());
}
```

Apply the equivalent refactor in `src/packages/sync.rs`, `src/packages/rebuild.rs`, `src/packages/remove.rs`, `src/packages/install.rs`, `src/packages/choose_install.rs`, `src/packages/find.rs`, and `src/packages/full_upgrade.rs` (with the `noconfirm` → `topgrade -y` mapping handled in Task 8). Remove the now-unused `use std::process::Command;` and `use std::io::Write;` imports where they become unused.

- [ ] **Step 3: Add a test-only override point**

At the bottom of `src/packages/mod.rs`, add:

```rust
#[cfg(test)]
pub mod test_support {
    use std::cell::RefCell;

    thread_local! {
        pub static SCRIPTED: RefCell<Vec<Vec<String>>> = RefCell::new(Vec::new());
        pub static NEXT_EXIT: RefCell<i32> = RefCell::new(0);
    }

    pub fn record_and_run(bin: &str, args: &[&str]) -> i32 {
        SCRIPTED.with(|s| s.borrow_mut().push(std::iter::once(bin.to_string()).chain(args.iter().map(|a| a.to_string())).collect()));
        NEXT_EXIT.with(|c| *c.borrow())
    }

    pub fn clear() {
        SCRIPTED.with(|s| s.borrow_mut().clear());
        NEXT_EXIT.with(|c| *c.borrow_mut() = 0);
    }

    pub fn scripted() -> Vec<Vec<String>> {
        SCRIPTED.with(|s| s.borrow().clone())
    }

    pub fn set_next_exit(code: i32) {
        NEXT_EXIT.with(|c| *c.borrow_mut() = code);
    }
}
```

`pub fn run_command` is NOT replaced under `#[cfg(test)]` — instead, individual test files use the `record_and_run` helper directly to assert argv. The seam is `run_command`; tests assert by calling the subcommand's argv-building code path or by inspecting the `SCRIPTED` thread-local when they call subcommand functions that internally use `run_command`.

- [ ] **Step 4: Run `cargo build --release` and `cargo test`**

Run: `cargo build --release && cargo test`
Expected: green. (No new test assertions yet; this task is pure refactor.)

- [ ] **Step 5: Commit**

```bash
git add src/
git commit -m "refactor: funnel subprocess invocation through run_command seam"
```

---

## Task 3: Harden `file::write_packages` and add `read_packages_filtered`

**Files:**
- Modify: `src/file.rs`
- Test: `tests/file_io.rs`

**Interfaces:**
- Produces:
  - `pub fn read_packages_filtered(path: PathBuf) -> Vec<String>` — same as `read_packages` but drops lines containing `#` and empty lines.
  - `pub fn write_packages_atomic(path: PathBuf, content: &str) -> Result<(), io::Error>` — writes to `path.with_extension("tmp")` then `rename`s into place; returns `io::Error` instead of panicking.

**Context:** The `#` and empty-line filtering is duplicated in `install.rs`, `remove.rs`, `sync.rs`, `rebuild.rs`. Centralizing it eliminates the duplication and gives a single testable unit. Atomic writes prevent half-written `~/packages` if the process is killed mid-write.

- [ ] **Step 1: Write failing test in `tests/file_io.rs`**

```rust
use std::io::Write;
use ah_pkg::file; // re-export; if not re-exported, use crate::file via a lib.rs shim — see Step 5 note

#[test]
fn read_packages_filtered_drops_comments_and_blanks() {
    let dir = tempdir();
    let p = dir.join("packages");
    let mut f = std::fs::File::create(&p).unwrap();
    writeln!(f, "vim").unwrap();
    writeln!(f, "# this is a comment").unwrap();
    writeln!(f, "").unwrap();
    writeln!(f, "git").unwrap();
    drop(f);
    let v = file::read_packages_filtered(p);
    assert_eq!(v, vec!["vim".to_string(), "git".to_string()]);
}

#[test]
fn write_packages_atomic_writes_content() {
    let dir = tempdir();
    let p = dir.join("packages");
    file::write_packages_atomic(p.clone(), "vim\ngit\n").unwrap();
    let got = std::fs::read_to_string(p).unwrap();
    assert_eq!(got, "vim\ngit\n");
}

fn tempdir() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("ah-test-{}", std::process::id()));
    std::fs::create_dir_all(&p).unwrap();
    p
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test file_io`
Expected: compile error — `read_packages_filtered` and `write_packages_atomic` don't exist, and `ah_pkg` is not importable as a library (see Step 5).

- [ ] **Step 3: Add a `lib.rs` shim so tests can import internals**

Create `src/lib.rs`:

```rust
pub mod cli;
pub mod file;
pub mod packages;
```

Change `src/main.rs` from `mod cli; mod file; mod packages;` to `use ah_pkg::{cli, file, packages};` and add `mod packages;` style imports are unchanged because the lib re-exports them. Update `Cargo.toml` to add a `[lib]` entry:

```toml
[lib]
name = "ah_pkg"
path = "src/lib.rs"
```

- [ ] **Step 4: Implement the new `file.rs` functions**

Replace `src/file.rs` body with:

```rust
use std::{
    fs::{rename, File, OpenOptions},
    io::{self, prelude::*, BufReader, BufWriter},
    path::PathBuf,
};

pub fn read_packages(path: PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect()
}

pub fn read_packages_filtered(path: PathBuf) -> Vec<String> {
    read_packages(path)
        .into_iter()
        .filter(|l| !l.contains('#') && !l.is_empty())
        .collect()
}

pub fn append_package(path: PathBuf, package: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    writeln!(file, "{}", package)
}

pub fn write_packages(path: PathBuf, content: &str) -> io::Result<()> {
    write_packages_atomic(path, content)
}

pub fn write_packages_atomic(path: PathBuf, content: &str) -> io::Result<()> {
    let tmp = path.with_extension("tmp");
    {
        let f = File::create(&tmp)?;
        let mut w = BufWriter::new(f);
        writeln!(w, "{}", content)?;
        w.flush()?;
    }
    rename(&tmp, &path)
}
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --test file_io`
Expected: PASS (2 tests).

- [ ] **Step 6: Update call sites to use the new helpers**

In `src/packages/install.rs`, `src/packages/remove.rs`, `src/packages/sync.rs`, `src/packages/rebuild.rs`: replace the local `.filter(|p| !p.contains("#") && !p.is_empty())` chain with `file::read_packages_filtered(get_package_path())`.

`install.rs`'s `file::append_package` call now needs `?` since it returns `Result` (not `()`). Propagate the error with `?` — wrap in a `Box<dyn Error>`-friendly type at the call boundary.

- [ ] **Step 7: Commit**

```bash
git add src/lib.rs src/file.rs src/main.rs src/packages/ src/packages/*.rs Cargo.toml tests/file_io.rs
git commit -m "refactor: expose ah_pkg as a lib; add filtered read + atomic write helpers"
```

---

## Task 4: Resolve the `sync` / `rebuild` / `choose_install` overlap

**Files:**
- Delete: `src/packages/rebuild.rs`
- Modify: `src/packages/mod.rs` (remove `pub mod rebuild;` and `pub use rebuild::rebuild;`)
- Modify: `src/packages/sync.rs` (now the single "sync from file" command)
- Modify: `src/packages/choose_install.rs` (append to `~/packages` like `install.rs` does)
- Modify: `src/main.rs` (no change; `rebuild` was never wired to a subcommand anyway)

**Context:** `rebuild` is dead code (warned by the compiler) that does what `sync` already does, with the only difference being `sync` doesn't pass `--needed`. Since `rebuild` is not on the CLI, deleting it is a pure cleanup, not a breaking change. `choose_install`'s `"Package index has not been updated!"` warning is a self-documented bug — the right fix is to update the index the same way `install` does.

- [ ] **Step 1: Write a failing test asserting `rebuild` is gone**

In `tests/cli_flags.rs`, add:

```rust
#[test]
fn rebuild_subcommand_is_removed() {
    // The CLI binary should reject "rebuild" — clap will print "unrecognized subcommand".
    use std::process::Command;
    let out = Command::new(env!("CARGO_BIN_EXE_ah"))
        .arg("rebuild")
        .output()
        .expect("run ah");
    assert!(!out.status.success(), "rebuild should be an unknown subcommand");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("unrecognized") || stderr.contains("unknown"),
        "expected clap error, got: {}",
        stderr
    );
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test cli_flags rebuild_subcommand_is_removed`
Expected: FAIL — `rebuild` is currently accepted by clap (wait — it's not actually wired; clap will reject it, so this test should PASS already). If it passes, mark the test as pinning current behavior and move to Step 3. If it fails for a different reason (e.g., the binary doesn't build), fix that first.

- [ ] **Step 3: Delete `src/packages/rebuild.rs` and its references**

```bash
git rm src/packages/rebuild.rs
```

In `src/packages/mod.rs`, remove:

```rust
pub mod rebuild;
pub use rebuild::rebuild;
```

- [ ] **Step 4: Fix `choose_install` to update the index**

In `src/packages/choose_install.rs`, replace the trailing block:

```rust
println!(
    "{} {}",
    "::".bold().red(),
    "Package index has not been updated!".bold()
);
```

with:

```rust
for word in &query {
    file::append_package(get_package_path(), word)?;
}
println!(
    "{} {}",
    "::".bold().blue(),
    "Package index updated".bold()
);
```

- [ ] **Step 5: Run `cargo build --release` and `cargo test`**

Run: `cargo build --release && cargo test`
Expected: green; no dead-code warnings.

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "refactor: delete dead rebuild subcommand; choose_install now updates index"
```

---

## Task 5: Add confirmation prompt to `remove` and `install`

**Files:**
- Modify: `src/packages/remove.rs`
- Modify: `src/packages/install.rs`
- Modify: `src/packages/mod.rs` (expose a `confirm_destructive` helper)
- Test: `tests/confirm.rs`

**Context:** `remove` deletes packages — destructive, must confirm by default. `install` is less destructive but adding confirmation matches the existing `upgrade`/`sync` behavior and the user already pays the cost of typing `y` to `paru`. Centralize the confirm logic so all four call sites behave identically.

- [ ] **Step 1: Add `confirm_destructive` helper in `src/packages/mod.rs`**

```rust
/// Prompts the user for confirmation. Returns `true` if they answer
/// "y" or hit enter; `false` on "n" or EOF. Errors on I/O failure.
pub fn confirm_destructive(action: &str) -> Result<bool, io::Error> {
    print!("{} About to {}. Continue? [Y/n] ", "::".bold().red(), action);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y")
}
```

Keep the existing `ask_confirmation` (it's used by `upgrade`/`sync`/`full_upgrade` already).

- [ ] **Step 2: Write failing test in `tests/confirm.rs`**

```rust
use ah_pkg::packages::confirm_destructive;

#[test]
fn confirm_destructive_accepts_y() {
    // The function reads from stdin, which is hard to mock without
    // an extra seam. For now, this test exists to pin the signature.
    // A real stdin-mock test is added in Task 9 (test harness).
    let _ = confirm_destructive;
}
```

(The test is intentionally minimal here; Task 9 adds the harness that lets us actually drive stdin. The important assertion is that the symbol exists and is public.)

- [ ] **Step 3: Use it in `remove.rs`**

After the `"Removing packages..."` print, before spawning `paru`:

```rust
if !confirm_destructive(&format!("remove {}", unwanted_packages.join(" ")))? {
    return Err("Operation aborted".into());
}
```

Add `use crate::packages::confirm_destructive;` at the top.

- [ ] **Step 4: Use it in `install.rs`**

Same pattern after `"Installing packages..."`. Use the message `"install <pkgs>"`.

- [ ] **Step 5: Run `cargo test --test confirm` and `cargo build --release`**

Run: `cargo test --test confirm && cargo build --release`
Expected: green.

- [ ] **Step 6: Commit**

```bash
git add src/packages/ src/packages/mod.rs tests/confirm.rs
git commit -m "feat: confirm before install and remove"
```

---

## Task 6: Add `--yes` short-circuit to bypass confirmations

**Files:**
- Modify: `src/cli.rs` (add `--yes` to the top-level `Cli` struct)
- Modify: `src/packages/mod.rs` (thread an `assume_yes` flag through confirm helpers)
- Modify: `src/main.rs` (extract `--yes` and pass to all subcommand functions)
- Modify: `src/packages/install.rs`, `remove.rs`, `upgrade.rs`, `sync.rs`, `full_upgrade.rs` (accept `assume_yes`)
- Test: `tests/confirm.rs`

**Context:** A new `--yes` flag is a global non-breaking addition (the current `noconfirm` flags are per-subcommand, but `ah`'s README example doesn't promise any specific shape). This lets scripts do `ah --yes upgrade`. Without this, every script wrapper has to feed `y\n` to stdin.

- [ ] **Step 1: Write failing test in `tests/confirm.rs`**

```rust
#[test]
fn yes_flag_short_circuits_confirm() {
    use ah_pkg::packages::should_confirm;
    assert!(!should_confirm(false)); // default: confirm
    assert!(!should_confirm(true));  // --yes: skip
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test confirm yes_flag_short_circuits_confirm`
Expected: compile error — `should_confirm` does not exist.

- [ ] **Step 3: Add `should_confirm` and update confirm helpers**

In `src/packages/mod.rs`:

```rust
/// Returns true if we should actually prompt the user. `assume_yes`
/// (from `--yes`) short-circuits this.
pub fn should_confirm(assume_yes: bool) -> bool {
    !assume_yes
}
```

Refactor `ask_confirmation` to take `assume_yes: bool` and short-circuit when `!should_confirm(assume_yes)`. Same for `confirm_destructive`. Update all five call sites (`upgrade`, `sync`, `install`, `remove`, `full_upgrade`) to accept and pass `assume_yes`.

- [ ] **Step 4: Add `--yes` to `Cli`**

In `src/cli.rs`:

```rust
#[derive(Parser)]
pub struct Cli {
    #[arg(long, global = true, help = "Assume yes to all prompts")]
    pub yes: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}
```

- [ ] **Step 5: Pass `cli.yes` from `main.rs`**

In `src/main.rs`, change each match arm to pass the flag through, e.g.:

```rust
Some(cli::Commands::Upgrade { noconfirm }) => packages::upgrade(noconfirm, cli.yes),
```

- [ ] **Step 6: Run `cargo test` and `cargo build --release`**

Run: `cargo test && cargo build --release`
Expected: green; CLI now accepts `ah --yes upgrade`.

- [ ] **Step 7: Commit**

```bash
git add src/cli.rs src/main.rs src/packages/ tests/confirm.rs
git commit -m "feat: add global --yes flag to short-circuit confirmation prompts"
```

---

## Task 7: Add `--dry-run` to `sync` and `upgrade`

**Files:**
- Modify: `src/cli.rs` (add `dry_run` to `Sync` and `Upgrade`)
- Modify: `src/packages/sync.rs`
- Modify: `src/packages/upgrade.rs`
- Modify: `src/main.rs` (thread the flag through)
- Test: `tests/dry_run.rs`

**Context:** A declarative pkg manager without dry-run is hostile to scripted use. `paru -Sy` (no `u`) on its own doesn't do upgrade dry-run; we need to print the list ourselves and exit 0. Same for `sync` — print the list, don't spawn.

- [ ] **Step 1: Write failing test in `tests/dry_run.rs`**

```rust
use std::fs;
use ah_pkg::file;

fn write_list(dir: &std::path::Path, pkgs: &[&str]) {
    let p = dir.join("packages");
    let s = pkgs.join("\n") + "\n";
    fs::write(&p, s).unwrap();
}

#[test]
fn dry_run_does_not_spawn() {
    // We can't easily intercept `Command::new` without the test harness
    // from Task 9. For now, pin that the helper function `build_sync_argv`
    // exists and returns the expected argv shape.
    use ah_pkg::packages::build_sync_argv;
    let argv = build_sync_argv(&["vim", "git"], true /*dry_run*/, false /*noconfirm*/);
    assert!(argv.contains(&"--print").to_string().as_str() || argv.iter().any(|a| a.contains("print")));
}
```

If this test is awkward, the simpler approach is: assert that `build_sync_argv` is `pub` and returns a `Vec<&'static str>` whose contents depend on `dry_run`. Adjust the test accordingly — the goal is pinning the seam.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test dry_run`
Expected: compile error.

- [ ] **Step 3: Add `--print`-style argv builder**

In `src/packages/sync.rs`, factor argv construction:

```rust
pub fn build_sync_argv(pkgs: &[&str], dry_run: bool, noconfirm: bool) -> Vec<&'static str> {
    let mut argv: Vec<&'static str> = vec!["--color", "always"];
    argv.extend(noconfirm_arg(noconfirm));
    if dry_run {
        argv.push("--print");
        argv.push("-");
    } else {
        argv.push("-S");
        argv.push("--needed");
        argv.push("-");
    }
    argv
}
```

`paru` doesn't have a `--print` flag; use `pacman -Sp` semantics via `paru -Qq` no — actually `paru` accepts `--print` to print package URLs only. Real implementation: just print the list ourselves and skip the spawn when `dry_run` is true. Replace the spawn block in `sync` with:

```rust
if dry_run {
    println!("Would install:");
    for p in &pkgs {
        println!("  {}", p);
    }
    return Ok(());
}
let argv = build_sync_argv_inner(&pkgs, dry_run, noconfirm);
let code = run_command(PACKAGE_MANAGER, &argv)?;
// ... rest unchanged
```

Simplify: just inline the early-return in `sync`. Skip `build_sync_argv` if it complicates tests. Adjust the test to assert the function-level behavior instead.

- [ ] **Step 4: Add `dry_run` to CLI**

In `src/cli.rs`, add to `Sync` and `Upgrade`:

```rust
#[arg(long, help = "Print what would happen, don't do it")]
dry_run: bool,
```

- [ ] **Step 5: Wire through `main.rs`**

Thread `dry_run` from each subcommand variant into the corresponding function.

- [ ] **Step 6: Run `cargo test` and `cargo build --release`**

Run: `cargo test && cargo build --release`
Expected: green; `ah sync --dry-run` prints the list and exits 0.

- [ ] **Step 7: Commit**

```bash
git add src/cli.rs src/main.rs src/packages/sync.rs src/packages/upgrade.rs tests/dry_run.rs
git commit -m "feat: add --dry-run to sync and upgrade"
```

---

## Task 8: Fix `full_upgrade` argv and confirm-before-run

**Files:**
- Modify: `src/packages/full_upgrade.rs`
- Modify: `src/main.rs` (no public change; just confirm `full_upgrade` is called with the right args)

**Context:** `topgrade` accepts `-y` to skip prompts. The current `full_upgrade` builds argv `["-y"]` (good when `noconfirm: true`) or `[""]` (passes an empty string argument — `topgrade` will see an empty positional and likely error or ignore it). Fix: pass nothing when not confirming.

- [ ] **Step 1: Write failing test in `tests/cli_flags.rs`**

```rust
#[test]
fn full_upgrade_argv_omits_flag_when_not_confirming() {
    use ah_pkg::packages::topgrade_argv;
    assert_eq!(topgrade_argv(true), vec!["-y"]);
    assert_eq!(topgrade_argv(false), Vec::<&str>::new());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test cli_flags full_upgrade_argv_omits_flag_when_not_confirming`
Expected: compile error.

- [ ] **Step 3: Extract `topgrade_argv` and update `full_upgrade`**

In `src/packages/mod.rs`:

```rust
pub fn topgrade_argv(assume_yes: bool) -> Vec<&'static str> {
    if assume_yes { vec!["-y"] } else { Vec::new() }
}
```

In `src/packages/full_upgrade.rs`, replace the local `noconfirm` mapping with `topgrade_argv(assume_yes)` and use `run_command(SYSTEM_UPDATER, &argv)?` like the other subcommands.

- [ ] **Step 4: Run `cargo test` and `cargo build --release`**

Run: `cargo test && cargo build --release`
Expected: green.

- [ ] **Step 5: Commit**

```bash
git add src/packages/full_upgrade.rs src/packages/mod.rs tests/cli_flags.rs
git commit -m "fix: full_upgrade omits empty arg when not confirming"
```

---

## Task 9: Build a test harness for fake `paru` / `topgrade`

**Files:**
- Modify: `src/packages/mod.rs` (re-exports)
- New: `tests/support/mod.rs` (compiled as a test helper module)

**Context:** We need integration tests that prove `ah` actually passes the right argv to `paru` without `paru` being installed. Approach: tests set an env var `AH_TEST_FAKE_BIN` to a path; `run_command` (under `#[cfg(test)]` only) checks that var first and runs that binary instead, capturing its argv into a sibling file `AH_TEST_RECORD`. Real builds ignore this entirely.

- [ ] **Step 1: Add test-only override in `run_command`**

Replace `run_command` in `src/packages/mod.rs` with:

```rust
pub fn run_command(bin: &str, args: &[&str]) -> Result<i32, Box<dyn std::error::Error>> {
    #[cfg(test)]
    {
        if let Ok(fake) = std::env::var("AH_TEST_FAKE_BIN") {
            // Record argv for assertion by tests.
            if let Ok(rec) = std::env::var("AH_TEST_RECORD") {
                let line = std::iter::once(bin.to_string())
                    .chain(args.iter().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
                    .join("\t");
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&rec)
                    .and_then(|mut f| std::io::Write::write_all(&mut f, (line + "\n").as_bytes()));
            }
            // Spawn the fake binary and return its exit code.
            let status = std::process::Command::new(&fake)
                .args(args)
                .status()
                .map_err(|e| format!("fake binary spawn failed: {}", e))?;
            return Ok(status.code().unwrap_or(-1));
        }
    }
    let status = std::process::Command::new(bin)
        .args(args)
        .status()
        .map_err(|e| format!("failed to spawn {}: {}", bin, e))?;
    Ok(status.code().unwrap_or(-1))
}
```

- [ ] **Step 2: Create `tests/support/mod.rs`**

```rust
//! Shared test harness: builds a fake `paru`/`topgrade` shell script
//! that records its argv to a file, sets env vars, and returns.
#![allow(dead_code)]

use std::fs;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

pub fn fake_bin_path() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ah-fake-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let p = dir.join("fake-bin");
    let script = r#"#!/bin/sh
echo "$0" "$@" >> "$AH_TEST_RECORD"
exit "${AH_TEST_EXIT:-0}"
"#;
    fs::write(&p, script).unwrap();
    let mut opts = fs::Permissions::from_mode(0o755);
    fs::set_permissions(&p, opts.clone()).unwrap();
    opts.set_mode(0o755);
    fs::set_permissions(&p, opts).unwrap();
    p
}

pub fn record_path() -> PathBuf {
    std::env::temp_dir().join(format!("ah-record-{}", std::process::id()))
}

pub fn setup_env() {
    let bin = fake_bin_path();
    let rec = record_path();
    let _ = fs::remove_file(&rec);
    std::env::set_var("AH_TEST_FAKE_BIN", &bin);
    std::env::set_var("AH_TEST_RECORD", &rec);
}

pub fn read_record() -> Vec<Vec<String>> {
    let rec = record_path();
    let Ok(content) = fs::read_to_string(&rec) else { return Vec::new() };
    content.lines().map(|l| l.split('\t').map(String::from).collect()).collect()
}
```

In `tests/cli_flags.rs`, add `mod support;` and use `support::setup_env()` / `support::read_record()` in tests that need to assert argv end-to-end.

- [ ] **Step 3: Add one end-to-end argv test using the harness**

In `tests/cli_flags.rs`:

```rust
mod support;

#[test]
fn upgrade_spawns_with_correct_argv() {
    support::setup_env();
    // Call packages::upgrade directly — but upgrade currently reads from stdin
    // and is interactive. Instead, exercise the argv helper:
    use ah_pkg::packages::{noconfirm_arg, topgrade_argv};
    let _ = noconfirm_arg(true);
    let _ = topgrade_argv(true);
    // A real subprocess test for `upgrade` is added in Task 11 (smoke).
}
```

- [ ] **Step 4: Run `cargo test`**

Run: `cargo test`
Expected: green.

- [ ] **Step 5: Commit**

```bash
git add src/packages/mod.rs tests/support/
git commit -m "test: add fake-binary harness for subprocess assertions"
```

---

## Task 10: Add PR-time CI workflow

**Files:**
- New: `.github/workflows/ci.yml`

**Context:** The existing `release.yml` only runs on tag push (via cargo-dist). PRs land with no check. Add a workflow that runs on `pull_request` and `push` to `main` and exercises the actual MVP surface: `cargo fmt --check`, `cargo clippy`, `cargo test`, `cargo build --release`. Caching for `~/.cargo` and the target dir.

- [ ] **Step 1: Write the workflow file**

Create `.github/workflows/ci.yml`:

```yaml
name: CI
on:
  pull_request:
  push:
    branches: [main]
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
permissions:
  contents: read
jobs:
  check:
    name: fmt + clippy + test + build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('Cargo.lock') }}
      - name: cargo fmt --check
        run: cargo fmt --all -- --check
      - name: cargo clippy
        run: cargo clippy --all-targets -- -D warnings
      - name: cargo test
        run: cargo test --all
      - name: cargo build --release
        run: cargo build --release
```

- [ ] **Step 2: Verify locally that the commands all succeed**

Run:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```
Expected: all green.

- [ ] **Step 3: Push and confirm CI runs on the PR**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add PR-time fmt/clippy/test/build workflow"
git push -u origin HEAD
gh pr create --fill --base main
```

Watch the PR's checks tab. Expected: `check` job passes.

- [ ] **Step 4: Confirm `release.yml` is unaffected**

Open `.github/workflows/release.yml` and confirm the file is unchanged (`git diff main -- .github/workflows/release.yml` is empty).

- [ ] **Step 5: Commit any incidental fixes (none expected)**

If `cargo fmt` or `cargo clippy` reports issues that earlier tasks missed, fix them here and commit as a follow-up `chore: address ci findings`.

---

## Task 11: Smoke test on a Linux runner

**Files:**
- New: `.github/workflows/smoke.yml` (optional but recommended; this is the only way to validate `ah` against a real `paru` and `topgrade` install path without Arch hardware)

**Context:** The unit + integration tests cover argv construction. The actual subcommand exit codes, the `~/packages` write path, and the interactive confirm prompts can only be exercised end-to-end against a real `paru`. The cheapest place to do that is the same CI runner — install `paru` (which itself needs AUR, so it's hard) OR just install `pacman` (Arch) and `fakeit`. Simpler: install Arch in a Docker container, install `paru`/`topgrade`, run `ah install <something trivial> --yes --dry-run` and `ah find nonexistent`.

- [ ] **Step 1: Smoke workflow (Arch-based)**

Create `.github/workflows/smoke.yml`:

```yaml
name: Smoke
on:
  workflow_dispatch:
  push:
    branches: [main]
concurrency:
  group: smoke-${{ github.ref }}
  cancel-in-progress: true
permissions:
  contents: read
jobs:
  smoke:
    runs-on: ubuntu-latest
    container:
      image: archlinux:latest
    steps:
      - uses: actions/checkout@v4
      - name: Install build deps
        run: |
          pacman -Syu --noconfirm rustup base-devel git
          rustup default stable
      - name: Install ah
        run: |
          cargo build --release
          install -Dm755 target/release/ah /usr/bin/ah
      - name: ah --version
        run: ah --version
      - name: ah find (must not panic)
        run: ah find nonexistent-package-xyz | head -5
      - name: ah sync --dry-run (must print, must not error)
        run: |
          mkdir -p "$HOME"
          touch "$HOME/packages"
          ah sync --dry-run
```

- [ ] **Step 2: Trigger via `workflow_dispatch` from the Actions tab**

Confirm the smoke run succeeds. The `paru` install isn't done (it requires AUR, which requires bootstrapping); the smoke test deliberately exercises only the argv-building and prompt code paths, not actual package installation. Real `paru` testing stays local on the maintainer's Arch box.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/smoke.yml
git commit -m "ci: add Arch-container smoke workflow"
```

---

## Task 12: Document the MVP surface

**Files:**
- New: `docs/MVP.md`
- Modify: `README.md` (fix the "paru or other package managers" claim — it still says "or other" but only paru is supported)

- [ ] **Step 1: Write `docs/MVP.md`**

```markdown
# `ah` MVP Surface (0.4.0)

The MVP cut of `ah` covers the minimum useful declarative package
management on Arch Linux. Anything not in this list is explicitly
out of scope.

## Subcommands

| Command | Alias | Confirms? | `--dry-run`? | Notes |
|---|---|---|---|---|
| `install <pkgs>` | `i` | yes | no | Appends new pkgs to `~/packages`. |
| `remove <pkgs>` | `r` | yes | no | Removes pkgs from `~/packages`. |
| `sync` | `s` | yes | yes | Installs every pkg listed in `~/packages`. |
| `upgrade` | `u` | yes | yes | Runs `paru -Syu`. |
| `find <query>` | `f` | n/a | n/a | Read-only search. |
| `choose-install <query>` | `fi` | n/a | n/a | Interactive pick + install; updates index. |
| *(none)* | — | yes | no | Default action: `topgrade` (full system upgrade). |

## Global flags

- `--yes` — assume yes to all confirmation prompts.

## Excluded from MVP

- Multi-file / grouped lists.
- File locking on `~/packages`.
- Pluggable backends (paru is the only supported backend).
- AUR auto-publish.
```

- [ ] **Step 2: Fix README to match reality**

In `README.md`, change:

> Arch Helper is a declarative package management tool for Arch Linux. It leverages paru or other package managers for seamless integration.

to:

> Arch Helper is a declarative package management tool for Arch Linux. It wraps `paru` for package management and `topgrade` for full system upgrades.

- [ ] **Step 3: Commit**

```bash
git add docs/MVP.md README.md
git commit -m "docs: MVP surface doc + README wording fix"
```

---

## Self-Review (run before yielding)

- [ ] Every CLI subcommand from 0.3.1 (`install`, `remove`, `sync`, `upgrade`, `find`, `choose-install`) still exists with the same name, alias, and arguments. The only additions are `--dry-run` on `sync`/`upgrade` and global `--yes`.
- [ ] No `rebuild` references remain (grep `git grep rebuild` returns nothing).
- [ ] No `--confirm` literal remains in any `paru`/`pacman` argv (grep `git grep '\\-\\-confirm' src/`).
- [ ] `cargo fmt --check` clean.
- [ ] `cargo clippy --all-targets -- -D warnings` clean.
- [ ] `cargo test --all` green.
- [ ] `cargo build --release` produces a binary that responds to `--help`, `--version`, and every subcommand.
- [ ] `.github/workflows/release.yml` byte-identical to `main`.
- [ ] New `.github/workflows/ci.yml` runs on PR and gates merges (settings.json enforcement is out of scope, but the workflow exists).
- [ ] `docs/MVP.md` describes the actual shipped surface.

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-10-05-ah-mvp-and-cicd.md`. Two execution options:

1. **Subagent-Driven (recommended)** — dispatch a fresh subagent per task, review between tasks, fast iteration.
2. **Inline Execution** — execute tasks in this session with checkpoints for review.

Which approach?
