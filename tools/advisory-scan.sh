#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p .work/tmp
export TMPDIR="$PWD/.work/tmp"
exec python3 tools/advisory_scan.py
