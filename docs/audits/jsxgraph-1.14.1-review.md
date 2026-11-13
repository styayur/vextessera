# JSXGraph 1.14.1 numeric boundary review

Source review: 3 November 2026. Scope: the exact 1.13.3 to 1.14.1 update requested in dependency PR #39, with the existing numeric-only renderer. No expression interpreter, general-purpose JSXGraph entry, CSP exception or remote script permission is enabled.

The original 1.13.3 npm archive was verified against its registry SHA-512 integrity. Its source was compared with the lock-bound installed 1.14.1 package. The transitive static import closure of the eight existing numeric entry modules remains the same 42 source files. JessieCode and Geonext remain replaced by the existing interfaces whose expression methods throw. No new source module enters this closure.

After TypeScript parsing/printing with comments removed, 11 files have semantic changes. The entire 433-line normalized diff was reviewed:

| Area | Reviewed changes |
| --- | --- |
| Board | Hidden-container sizing, latest ResizeObserver dimensions, object removal bookkeeping and dependency-display formatting |
| Constants | Version and displayed attribution text |
| Elements/text/polygon | Circle bounds, preservation of explicit false/default text attributes, polygon formatting |
| Math/numerics | Bounded integer GCD/LCM handling, missing Romberg configuration and Gauss-Legendre parity correction |
| Options | Invisible, unnamed generated parent points |
| Canvas/SVG | Nonnegative gradient radii, border radius handling, safe connected-node layer movement and undefined-point loop progress |
| Type helpers | Explicit default attributes and integer/decimal zero trimming |

The ES6 probe in `utils/env.js`, dynamic expression boundary, `reader/file.js` and numeric entry imports have no semantic changes. The exact-probe replacement remains fail-closed. MIT and LGPL license files are byte-identical to 1.13.3; SHA-256 values are `43e325001487de7f09745fbe086b054bb1b70d4ab58060e87153849365b23ff1` and `e3a994d82e644b03a792a930f574002658412f62407f5fee083f2555c5f23118`. No upstream source or license text is modified.

Only the reviewed exact version is accepted by the build and final bundle provenance checks. Original parser-module exclusion, executable JavaScript AST prohibition, source-string boundary rejection and real browser tests with global eval/Function blocked remain required. Actual SVG/numeric lesson controls, pointer/keyboard interactions, cleanup, localization, reduced motion, narrow layout and the deployed-subpath build must pass before merging. This source review does not substitute for those runtime checks and is not a claim of general-purpose static security analysis.

Local acceptance passed with the updated React/DOM 19 pair: complete unit/migration tests, 45 Rust core tests, 23 browser groups, all 24 lesson states in three locales, and the real Pages-subpath build. The final bundle gate parsed 15 JavaScript assets with no executable eval/Function syntax and no original parser modules. Native compilation, three-platform runtime contracts, retrieval parity and actual OpenGL scenes remain required CI gates.
