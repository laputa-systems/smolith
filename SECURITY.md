# Security

Report a vulnerability through [GitHub private advisories](https://github.com/laputa-systems/smolith/security/advisories/new). An unfixed vulnerability does not belong in a public issue.

## Advisory checks

The packages are named `laputa-<upstream-name>`, while RustSec records their upstream names. [UPSTREAM.toml](UPSTREAM.toml) owns that mapping and the imported versions. Scanning only the renamed packages misses those upstream advisories.

`tools/advisory-scan.sh` checks the mapped upstream versions and locked registry dependencies. Normal and build dependencies reached with all features count as production, including opt-in features. Vulnerabilities and unsound advisories affecting the scanner's Linux/macOS/Unix scope fail the scan. Dev-only, other informational, and other-platform matches are printed without failing it.

The scan refreshes the advisory database and prints its revision and commit date. `--offline` uses the existing database checkout and labels that choice; it does not supply uncached Cargo dependencies. [docs/LEDGER.md](docs/LEDGER.md) is a historical advisory and upstream-issue snapshot, not the result of a current scan. Imported version checks also do not establish that local modifications are sound.
