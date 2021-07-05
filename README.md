# Video subtitle generation
A command line programme to generate [SRT format](https://en.wikipedia.org/wiki/SubRip) subtitles for use in video players.

The subtitles are generated from the full transcript **text** of a video.  By default, each subtitle will have a maximum of 10 words.

## Current status
Youngling.  Actively-developed.

## Usage
### CLI
```
$ subtitles INPUT-FILENAME.txt OUTPUT-FILENAME.srt
```

### Webassembly
```
import init, * as subtitles from 'path/to/subtitles.js';

(async () => await init())();

let transcript_text = 'LOTS OF TEXT GOES HERE';
let audio_length_in_seconds = 99;

const text_for_srt_file = subtitles.prepare_srt_content(transcript_text, audio_length_in_seconds);
```

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
