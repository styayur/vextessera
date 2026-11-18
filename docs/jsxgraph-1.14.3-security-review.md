# JSXGraph 1.14.3 review

The 1.14.2 to 1.14.3 source comparison is available at
https://github.com/jsxgraph/jsxgraph/compare/v1.14.2...v1.14.3.
This is a patch release, with changes to board resize handling, geometry,
text, transforms, documentation and generated distributions. The application
continues to import the numeric source modules, not the distributed bundle.
The comparison does not change `src/utils/env.js`, `src/parser/jessiecode.js`
or `src/parser/geonext.js`.

The exact-version allowlist advances to 1.14.3. Both executable parsers remain
replaced by local disabled implementations. The ES6 dynamic-compilation probe
still must match exactly before replacement; a changed probe fails the build.
The emitted provenance still forbids every upstream parser module. The bundle
AST check still rejects global eval and Function calls, including indirect
forms. The version assertion in that check advances together with the build.

Required compatibility evidence is the desktop and web builds, unit tests,
the executable-AST bundle check, and the browser regression covering authored
visualizations. A version allowlist change alone is not verification. See the
PR checks for actual results. The separate picomatch development-tool advisory
GHSA-fc5q-39xc-vjpf remains subject to the unchanged npm audit gate; this review
does not waive or resolve that advisory.
