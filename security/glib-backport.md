# GTK3 GLib security backport

Tauri/wry's Linux GTK3 stack uses glib 0.18.5. Replacing it with glib 0.20
would introduce different Rust types/GTK ABI bindings and does not satisfy the
GTK3 dependency requirements. Windows does not use this GTK implementation.

The MIT-licensed crate in `vendor/glib` is the exact crates.io 0.18.5 archive
(checksum in `glib-provenance.json`) plus upstream gtk-rs-core commit
`b5a4071e439bef2b5eea76c3aa25e5ae84839e34`: declare the output pointer mutable
and pass `&mut p` to `g_variant_get_child`. Its public API/version are retained.
An empty Cargo workspace isolates upstream lint/dev tooling. Upstream MIT notices
and dependency requirements are retained. The additional reviewed local changes
below are recorded separately from the two-line upstream fix.

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

## Additional local FFI and ownership hardening

The initial exact backport exposed seven CodeQL findings in the newly vendored
source. Its traces connect charset/GError/boolean outputs through generic
conversion helpers to incompatible GObject/GValue/boxed/log-level types. The
local delta names `Error`, `GString`, `PathBuf` and `bool` conversion targets
explicitly at those C boundaries. All 64 matching GError conversion sites across
ten source files use the concrete Error conversion, including key-file,
URI, markup, variant, encoding, thread-pool and manual helpers. This retains the same native calls, null/error
branches, optional hostname handling and ownership transfers.

`PtrSlice::truncate` now reads the owned element, writes the NULL slot and only
then drops the owned element. The enclosing GLib allocation stays owned by the
slice, and the terminator is established before user destruction can unwind.
The unknown-log-level panic also uses a constant diagnostic instead of emitting
foreign numeric bits. No unsafe field, test or scan scope is removed.

`glib-local-hardening.patch` and its checksum/base hashes in the provenance
record distinguish these changes from upstream. The Linux harness also checks
UUID validation, console charset, Unicode filename URI round trips, native
GError paths and object destruction/refcounts during slice truncation. These
tests also cover key-file/parser errors and valid Unicode values. Enabling
`v2_78` in the standalone Linux harness type-checks the full version-gated
binding surface on that runner; the application's feature graph is unchanged.
These tests supplement the original 1000-iteration optimized iterator regression;
fresh native/security results are required for each source revision.

Rust's [read ownership](https://doc.rust-lang.org/std/ptr/fn.read.html) and
[write contract](https://doc.rust-lang.org/std/ptr/fn.write.html) explain the
move/write/drop order. GLib documents the
[boolean UUID-validation result](https://docs.gtk.org/glib/func.uuid_string_is_valid.html).
These source changes do not waive the CodeQL findings: the unchanged gate must
analyze the new revision.

RUSTSEC-2024-0429 / GHSA-wrw7-89jp-8q8g formally marks all glib <0.20 affected.
This is a verified source backport, not a claim that upstream published a fixed
GTK3 release. Do not dismiss the advisory, change the package version to evade
it, or add audit ignores. Any remaining version-based alert stays visible until
upstream metadata/dependencies can represent the fix.

References:
- https://github.com/advisories/GHSA-wrw7-89jp-8q8g
- https://github.com/gtk-rs/gtk-rs-core/pull/1343
