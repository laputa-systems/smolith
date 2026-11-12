#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
sh scripts/check-features.sh
sh scripts/check-normal-dependencies.sh
