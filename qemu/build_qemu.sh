#!/bin/sh
#
# Baeum QEMU build script
#
# Copyright 2016 Choongwoo Han <cwhan.tunz@gmail.com>.
# Copyright 2016 KAIST SoftSec.
#
# from afl-fuzz QEMU mode
# --------------------------------------
#
# Written by Andrew Griffiths <agriffiths@google.com> and
#            Michal Zalewski <lcamtuf@google.com>
#
# Copyright 2015, 2016 Google Inc. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at:
#
#   http://www.apache.org/licenses/LICENSE-2.0
#

build_qemu () {
    ORIG_CPU_TARGET="$CPU_TARGET"

    test "$CPU_TARGET" = "" && CPU_TARGET="`uname -m`"
    test "$CPU_TARGET" = "i686" && CPU_TARGET="i386"

    echo "[*] Configuring QEMU for $CPU_TARGET..."

    cd qemu-2.8.0 || exit 1

    CFLAGS="-O3" ./configure --disable-system --enable-linux-user \
      --disable-gtk --disable-sdl --disable-vnc \
      --target-list="${CPU_TARGET}-linux-user" || exit 1

    echo "[+] Configuration complete."

    echo "[*] Attempting to build QEMU (fingers crossed!)..."

    make || exit 1

    echo "[+] Build process successful!"

    echo "[*] Copying binary..."
    cp -f "${CPU_TARGET}-linux-user/qemu-${CPU_TARGET}" "../qemu-trace" || exit 1
    cd ..
}

QEMU_URL="http://wiki.qemu-project.org/download/qemu-2.8.0.tar.bz2"
QEMU_SHA256="dafd5d7f649907b6b617b822692f4c82e60cf29bc0fc58bc2036219b591e5e62"

echo "========================================="
echo "Chatkey instrumentation QEMU build script"
echo "========================================="
echo

echo "[*] Performing basic sanity checks..."

if [ ! "`uname -s`" = "Linux" ]; then

  echo "[-] Error: QEMU instrumentation is supported only on Linux."
  exit 1

fi

if [ ! -f "patches/baeum.cc" ]; then

  echo "[-] Error: key files not found - wrong working directory?"
  exit 1

fi

T=`which apt-get 2>/dev/null`
if [ "$T" = "" ]; then
    echo "[-] Error: Sorry, this script needs apt-get."
    exit 1
fi

echo "[*] Download dependencies."
echo "sudo apt-get --no-install-recommends -qq -y build-dep qemu"
sudo apt-get --no-install-recommends -qq -y build-dep qemu
echo "sudo apt-get install -qq -y wget flex bison libtool automake autoconf autotools-dev pkg-config libglib2.0-dev"
sudo apt-get install -qq -y wget flex bison libtool automake autoconf autotools-dev pkg-config libglib2.0-dev

echo "[+] All checks passed!"

ARCHIVE="`basename -- "$QEMU_URL"`"

CKSUM=`sha256sum -- "$ARCHIVE" 2>/dev/null | cut -d' ' -f1`

if [ ! "$CKSUM" = "$QEMU_SHA256" ]; then

  echo "[*] Downloading QEMU 2.8.0 from the web..."
  rm -f "$ARCHIVE"
  wget -O "$ARCHIVE" -- "$QEMU_URL" || exit 1

  CKSUM=`sha256sum -- "$ARCHIVE" 2>/dev/null | cut -d' ' -f1`

fi

if [ "$CKSUM" = "$QEMU_SHA256" ]; then

  echo "[+] Cryptographic signature on $ARCHIVE checks out."

else

  echo "[-] Error: signature mismatch on $ARCHIVE (perhaps download error?)."
  exit 1

fi

echo "[*] Uncompressing archive (this will take a while)..."

rm -rf "qemu-2.8.0" || exit 1
tar xf "$ARCHIVE" || exit 1

echo "[+] Unpacking successful."

echo "[*] Build qemu-trace-coverage..."
echo "[*] Applying patches..."

patch -p0 <patches/qemu.patch || exit 1
cp patches/baeum.cc qemu-2.8.0/

echo "[+] Patching done."

build_qemu

cp -f "./qemu-trace" "../qemu-trace-coverage" || exit 1
rm -rf "./qemu-trace"
echo "[+] Successfully created '../qemu-trace-coverage'."

rm -rf "qemu-2.8.0"
rm -rf "qemu-2.8.0.tar.bz2"

exit 0
