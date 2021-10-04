//! Prepares a subtitle file (i.e. .srt file) from given text.
use crate::placeholder::Placeholders;
use std::collections::HashMap;
use std::fmt::Write;
use unicode_segmentation::UnicodeSegmentation; // 1.7.1

struct SrtEntry<'a> {
    counter: u32,
    subtitle: &'a str,
    start: f32,
    end: f32,
}

/// Plain text to SRT content.
///
/// Given a long blob of text, turns it into an SRT file content.
pub fn prepare_srt_content(
    text: &str,
    length_in_seconds: u32,
    abbr_map: &HashMap<String, String>,
) -> String {
    let mut srt_entry_list: Vec<SrtEntry> = Vec::new();

    let placeholders = Placeholders::new(abbr_map);
    let text_with_placeholders = placeholders.insert(text);

    let subtitle_list = crate::subtitle::split_into_subtitles(&text_with_placeholders);
    let total_grapheme_count = get_grapheme_count(&text_with_placeholders, &placeholders);
    let duration_per_grapheme: f32 = length_in_seconds as f32 / total_grapheme_count as f32;
    let mut last_subtitle_start_time = 0 as f32;

    for (i, subtitle) in subtitle_list.iter().enumerate() {
        let srt_entry = prepare_srt_entry(
            subtitle,
            last_subtitle_start_time,
            duration_per_grapheme,
            i as u32,
            &placeholders,
        );

        last_subtitle_start_time = srt_entry.end;

        srt_entry_list.push(srt_entry);
    }

    let srt_content_w_placeholders: String = srt_entries_to_str(srt_entry_list);
    let srt_content = placeholders.replace(&srt_content_w_placeholders);

    return srt_content;
}

/// Prepare a single entry for a single subtitle.
fn prepare_srt_entry<'a>(
    subtitle: &'a str,
    start: f32,
    duration_per_grapheme: f32,
    index: u32,
    placeholder_manager: &Placeholders,
) -> SrtEntry<'a> {
    let counter = index + 1;
    let grapheme_count = get_grapheme_count(&subtitle, placeholder_manager);
    let subtitle_duration = duration_per_grapheme * grapheme_count as f32;
    let end = start + subtitle_duration;

    return SrtEntry {
        counter,
        subtitle,
        start,
        end,
    };
}

/// Prints an SRT file into a *string*.
///
/// @see https://en.wikipedia.org/wiki/SubRip
fn srt_entries_to_str(srt_entry_list: Vec<SrtEntry>) -> String {
    let mut srt_content: String = String::new();

    for srt_entry in srt_entry_list {
        let start_min = srt_entry.start as u32 / 60;
        let start_sec = srt_entry.start % 60 as f32;
        let end_min = srt_entry.end as u32 / 60;
        let end_sec = srt_entry.end % 60 as f32;

        let _ = writeln!(&mut srt_content, "{}", srt_entry.counter);

        // The format string cannot use commas as decimal separators.
        // Hence the gymnastics.
        let time_entry = format!(
            "00:{:02}:{:06.3} --> 00:{:02}:{:06.3}\n",
            start_min, start_sec, end_min, end_sec
        )
        .replace('.', ",");
        srt_content.push_str(&time_entry);

        let _ = writeln!(&mut srt_content, "{}\n", srt_entry.subtitle);
    }

    return srt_content;
}

/// Determine grapheme count for the purpose of subtitling.
///
/// Replaces abbreviations with their fullforms before the count.
/// Ignores anything that is not considered alphanumeric.
///
/// @see https://en.wikipedia.org/wiki/Grapheme
/// @see https://en.wikipedia.org/wiki/Alphanumeric
fn get_grapheme_count(text: &str, placeholders: &Placeholders) -> u32 {
    let mut text_fullform = placeholders.expand(text);

    text_fullform.retain(|c| c.is_alphanumeric());

    return text_fullform.graphemes(true).count() as u32;
}
