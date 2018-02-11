#!/bin/bash

set -eu -o pipefail
exe=${1:?First argument it the executable under test}

root="$(cd "${0%/*}" && pwd)"
# exe="$(cd "${exe%/*}" && pwd)/${exe##*/}"

# shellcheck source=./tests/book-helpers.sh
source "$root/book-helpers.sh"

SUCCESSFULLY=0

fixture="$root/fixtures"
snapshot="$fixture/snapshots"

title "termbook build"

(sandboxed
  (with "rewrite enabled"
    (with "no specifically marked code blocks"
      book="$fixture/book-no-markers"
      it "succeeds" && {
        expect_run $SUCCESSFULLY "$exe" build --rewrite $book
      }
      
      it "wrote the original books files without any insertions" && {
        expect_snapshot "$OUTPUT_DIR" "$snapshot/build-no-markers"
      }
    )
  )
)

