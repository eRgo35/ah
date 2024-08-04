# Maintainer: Michał Czyż <mike@c2yz.com>
pkgname=ah
pkgver=0.2.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="A declarative package manager for Arch Linux"
url="https://github.com/eRgo35/ah"
license=('MIT')

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://github.com/eRgo35/ah
}
