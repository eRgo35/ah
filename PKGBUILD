# Maintainer: Michał Czyż <mike@c2yz.com>

pkgname=ah-pkg-bin
pkgver=0.4.0
pkgrel=1
pkgdesc="A declarative package manager for Arch Linux"
url="https://github.com/eRgo35/ah"
license=("MIT")
arch=("x86_64")
provides=("ah-pkg")
conflicts=("ah-pkg")
depends=("paru" "topgrade")
source=("https://github.com/eRgo35/ah/releases/download/v$pkgver/ah-pkg-x86_64-unknown-linux-gnu.tar.xz")
sha256sums=("SKIP")

package() {
    cd "$srcdir/ah-pkg-x86_64-unknown-linux-gnu"

    install -Dm755 ah -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE.md "$pkgdir/usr/share/licenses/$pkgname/LICENSE.md"
}
