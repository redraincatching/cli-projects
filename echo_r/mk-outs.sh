#!/usr/bin/env bash
# note, this won't actually work on windows, rewriting it was just an exercise for myself

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "hello there" > $OUTDIR/hello1.txt
echo -n "hello there" > $OUTDIR/hello1.n.txt
echo "hello    there" > $OUTDIR/hello2.txt
echo -n "hello    there" > $OUTDIR/hello2.n.txt