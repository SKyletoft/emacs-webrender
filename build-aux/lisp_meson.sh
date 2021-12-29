#!/bin/sh

# aux used in src/meson.build.

( sed -n 's/^[ \t]*(load "\([^"]*\)".*/\1/p' $1 | \
  sed -e 's/$/.elc/' -e 's/\.el\.elc/.el/'; )
