# Video subtitle generation
A command line programme to generate [SRT format](https://en.wikipedia.org/wiki/SubRip) subtitles for use in video players including YouTube.

The subtitles are generated from the full transcript **text** of a video.  By default, each subtitle will have a maximum of 10 words.

## Current status
Actively-developed.

## Usage
### CLI
```
$ subtitles --transcript INPUT-FILENAME.txt --length LENGTH-IN-SECONDS --abbr ABBREVIATION-MAP-FILE.csv > OUTPUT-FILENAME.srt
$ subtitles --transcript transcript.txt --length 300 --abbr abbreviations.csv > subtitles.srt # True example
```

### Webassembly
```
import init, * as subtitles from 'path/to/subtitles.js';

(async () => {
  await init();

  let transcript_text = 'Lots of text goes here; newline is acceptable.';
  let audio_length_in_seconds = 99;
  let abbreviation_map = new Map([["UNGA", "United Nations General Assembly"], ["MDN", "Mozilla Developer Network"]]); // Optional.

  const text_for_srt_file = subtitles.wasm_prepare_srt_content(transcript_text, audio_length_in_seconds, abbreviation_map);
})()
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

## Abbreviation map
The abbreviation map is an *optional* feature.  This is provided as a **CSV file** (with no header row) from the command line or as a Javascript Map object when using the Wasm version.  Its purpose is to map those character or character sequences who are pronounced very differently from how they appear.  For example, the transcript may use "(UNGA)" whereas the narrator may say "United Nations General Assembly" instead.  Another example is the Unicode codepoint [U+FDFD](https://en.wikipedia.org/wiki/Basmala#Unicode) which is spoken as a full sentence.  In all these cases, the written form in the transcript do not hint how long it may take to say it.  By providing a mapping between the written form and the spoken form, we get more accurate timing for each subtitle.

### Example CSV
```
(UNGA),(United Nations General Assembly)
(MDN),(Mozilla Developer Network)
﷽,Bi-smi llāhi r-raḥmāni r-raḥīm
```

## Licence
[Simplified BSD licence](https://spdx.github.io/license-list-data/BSD-2-Clause.html)
