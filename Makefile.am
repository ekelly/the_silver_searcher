ACLOCAL_AMFLAGS = ${ACLOCAL_FLAGS}

bin_PROGRAMS = ag
ag_SOURCES = src/ignore.c src/ignore.h src/log.c src/log.h src/options.c src/options.h src/print.c src/print.h src/scandir.c src/scandir.h src/search.c src/search.h src/lang.c src/lang.h src/util.c src/util.h src/decompress.c src/decompress.h src/uthash.h src/main.c src/rgzip.h

# for macs
ag_LDADD = ${PCRE_LIBS} ${LZMA_LIBS} ${ZLIB_LIBS} $(PTHREAD_LIBS) -L lib/ -lrgzip -lSystem -lpthread -lc -lm

# for linux
#ag_LDADD = ${PCRE_LIBS} ${LZMA_LIBS} ${ZLIB_LIBS} $(PTHREAD_LIBS) -L lib/ -lrgzip -ldl -lpthread -lrt -lgcc_s -lpthread -lc -lm

dist_man_MANS = doc/ag.1

bashcompdir = $(pkgdatadir)/completions
dist_bashcomp_DATA = ag.bashcomp.sh

EXTRA_DIST = Makefile.w32 LICENSE NOTICE the_silver_searcher.spec README.md

test: ag
	cram -v tests/*.t

test_big: ag
	cram -v tests/big/*.t

.PHONY : all test clean
