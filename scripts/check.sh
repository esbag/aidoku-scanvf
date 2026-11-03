#!/usr/bin/env bash
# Type-check every source on the host target (no wasm toolchain needed).
# "no global memory allocator" / "panic_handler" errors are expected outside wasm.
set -uo pipefail
cd "$(dirname "$0")/.."
host="$(rustc -vV | sed -n 's/^host: //p')"
status=0
for source in sources/*/; do
  [[ -f "$source/Cargo.toml" ]] || continue
  echo "== $source"
  out="$(cd "$source" && cargo check --target "$host" --color never 2>&1)"
  errors="$(grep -E '^error' <<<"$out" | grep -vE 'global memory allocator|panic_handler|could not compile' || true)"
  if [[ -n "$errors" ]]; then
    echo "$out"; status=1
  else
    grep -E '^warning' <<<"$out" || true
    echo "OK"
  fi
done
exit $status
