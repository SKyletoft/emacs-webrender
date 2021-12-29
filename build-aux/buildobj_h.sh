#!/bin/sh

# aux used in src/Makefile buildobj.h.

for i in $@; do \
  echo "$i" | sed 's,.*/,,; s/\.obj$/\.o/; s/^/"/; s/$/",/' \
    || exit; \
done
