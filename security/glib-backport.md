# GTK3 GLib security backport

Tauri/wry's Linux GTK3 stack uses glib 0.18.5. Replacing it with glib 0.20
would introduce different Rust types/GTK ABI bindings and does not satisfy the
GTK3 dependency requirements. Windows does not use this GTK implementation.

The MIT-licensed crate in `vendor/glib` is the exact crates.io 0.18.5 archive
(checksum in `glib-provenance.json`) plus upstream gtk-rs-core commit
`b5a4071e439bef2b5eea76c3aa25e5ae84839e34`: declare the output pointer mutable
and pass `&mut p` to `g_variant_get_child`. Its public API/version are retained.
An empty Cargo workspace isolates upstream lint/dev tooling; no other upstream
source or dependency requirement was changed. Upstream MIT notices are retained.

The lockfile points to the local path patch. `verify_glib_backport.py` verifies
all vendored bytes. The optimized Linux regression exercises forward, reverse,
mixed and indexed string-variant iteration, including empty and Unicode values.
CI records Linux and Windows reverse dependency trees and uses `--locked` for
the regression harness. The installed Tauri dependency is the same patched path.

Vextessera applies `[patch.crates-io]` from `src-tauri/Cargo.toml` to
`../vendor/glib`. Verification also checks that application patch/lock identity
and the existing Rust license inventory agree. CI asserts that the all-features
Linux application tree reaches this local GTK3 dependency and that the Windows
MSVC reverse tree is empty. Windows desktop compile and existing browser,
privacy, three-language, runtime and retrieval checks remain in the normal CI.

The verification script and standalone regression harness were imported from
the Apache-2.0 codeforge-engine source at
`2b2550472a0dbcbf6468e6a3487ff0d7af137dc3`; their license is retained in
`licenses/supplemental/glib-backport-tools-APACHE-2.0.txt`. Application integration
checks and explicit test-output writing are the local helper adaptations. The
upstream GLib payload remains MIT with its original `LICENSE`/`COPYRIGHT` bytes.
Existing dependency notices and application AGPL-3.0-only terms are preserved.

RUSTSEC-2024-0429 / GHSA-wrw7-89jp-8q8g formally marks all glib <0.20 affected.
This is a verified source backport, not a claim that upstream published a fixed
GTK3 release. Do not dismiss the advisory, change the package version to evade
it, or add audit ignores. Any remaining version-based alert stays visible until
upstream metadata/dependencies can represent the fix.

References:
- https://github.com/advisories/GHSA-wrw7-89jp-8q8g
- https://github.com/gtk-rs/gtk-rs-core/pull/1343
