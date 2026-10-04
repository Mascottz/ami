#!/bin/sh
set -eu

cue=${CUE_BIN:-cue}
"$cue" vet -c language/schema.cue language/tables.json

if hook_error=$("$cue" vet -c language/schema.cue language/tables.json language/testdata/invalid.cue 2>&1); then
  printf '%s\n' 'I expected my invalid hooked-letter fixture to fail.' >&2
  exit 1
fi
case "$hook_error" in
  *"languages.0.hooked.0.lower"*) ;;
  *) printf '%s\n' 'I could not verify my hooked-letter rejection.' >&2; exit 1 ;;
esac

if subdot_error=$("$cue" vet -c language/schema.cue language/tables.json language/testdata/invalid-subdot.cue 2>&1); then
  printf '%s\n' 'I expected my invalid Igbo subdot fixture to fail.' >&2
  exit 1
fi
case "$subdot_error" in
  *"languages.1.subdots.1.lower"*) ;;
  *) printf '%s\n' 'I could not verify my Igbo subdot rejection.' >&2; exit 1 ;;
esac

printf '%s\n' 'I passed the CUE language-table checks.'
