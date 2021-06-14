# Video subtitle generation
A command line programme to generate [SRT format](https://en.wikipedia.org/wiki/SubRip) subtitles for use in video players.

## Current status
Youngling.  Actively-developed.

## Build instruction
### CLI
```
$ cargo build --release
```

### Webassembly
[Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).  Then...
```
$ wasm-pack build --target web
$ cp pkg/{subtitles_bg.wasm,subtitles.js} /path/to/website/js/
```

## Licence
[Simplified BSD licence](https://spdx.github.io/license-list-data/BSD-2-Clause.html)
