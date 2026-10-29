#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
exec python3 fixtures/h12tiny.py
