# Maintainer: Michał Czyż <mike@c2yz.com>

pkgname=ah-pkg-bin
pkgver=0.3.1
pkgrel=1
pkgdesc="A declarative package manager for Arch Linux"
url="https://github.com/eRgo35/ah"
license=("MIT")
arch=("x86_64")
provides=("ah-pkg")
conflicts=("ah-pkg")
depends=("paru" "topgrade")
source=("https://github.com/eRgo35/ah/releases/download/v$pkgver/ah-pkg-x86_64-unknown-linux-gnu.tar.xz")
sha256sums=("6d1778f508d42a3396e466bf44bd650c2dabcd9552f1f776c1c93019f0c52a68")

package() {
    cd "$srcdir/ah-pkg-x86_64-unknown-linux-gnu"

    install -Dm755 ah -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE.md "$pkgdir/usr/share/licenses/$pkgname/LICENSE.md"
}
