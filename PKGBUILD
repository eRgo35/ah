# Maintainer: Michał Czyż <mike@c2yz.com>
pkgname=ah
pkgver=0.1.0
pkgrel=1
pkgdesc="A declarative package manager for Arch Linux"
url="https://github.com/eRgo35/ah"
license=("MIT")
arch=("x86_64")
makedepends=("cargo")

pkgver() {
    (git describe --long --tags || echo "$pkgver") | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
    return 0
}

package() {
    cd ..
    usrdir="$pkgdir/usr"
    mkdir -p $usrdir
    cargo install --no-track --path . --root "$usrdir"
}

