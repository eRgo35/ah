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
