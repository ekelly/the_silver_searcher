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
cd rust-zlib
cargo build
cd -
cp rust-zlib/src/rzlib.h src/
cp rust-zlib/target/librzlib-* lib/
mv lib/librzlib-* lib/librzlib.a

aclocal $AC_SEARCH_OPTS && \
autoconf && \
autoheader && \
automake --add-missing && \
./configure "$@" && \
make -j4
