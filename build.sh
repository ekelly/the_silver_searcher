#!/bin/sh

cd `dirname $0`

AC_SEARCH_OPTS=""
# For those of us with pkg-config and other tools in /usr/local
PATH=$PATH:/usr/local/bin

# This is to make life easier for people who installed pkg-config in /usr/local
# but have autoconf/make/etc in /usr/. AKA most mac users
if [ -d "/usr/local/share/aclocal" ]
then
    AC_SEARCH_OPTS="-I /usr/local/share/aclocal"
fi

# build our rust
cd rust-gzip
cargo clean
cargo build --release
cd -
mkdir lib
cp rust-gzip/src/rgzip.h src/
cp rust-gzip/target/release/librgzip-* lib/
mv lib/librgzip-* lib/librgzip.a

aclocal $AC_SEARCH_OPTS && \
autoconf && \
autoheader && \
automake --add-missing && \
./configure "$@" && \
make -j4
