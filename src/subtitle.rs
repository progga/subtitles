//! Prepares subtitles from given text.

use unicode_segmentation::UnicodeSegmentation; // 1.7.1

/// Split the entire file into *potential* subtitles.
pub fn split_into_subtitles(file_content: &str) -> Vec<&str> {
    let mut subtitle_list: Vec<&str> = Vec::new();
    let sentence_list: Vec<&str> = file_content.unicode_sentences().collect();

    for sentence in sentence_list {
        let mut subtitles_for_a_sentence = prepare_subtitles_from_sentence(sentence);
        subtitle_list.append(&mut subtitles_for_a_sentence);
    }

    return subtitle_list;
}

/// Split a sentence into subtitles.
///
/// Each sentence will produce a maximum of 5 subtitles.
///
/// Each subtitle contains 10 words.  The last subtitle is an exception.
///
/// @todo Split into multiple functions and house them in a separate module.
fn prepare_subtitles_from_sentence<'a>(sentence: &'a str) -> Vec<&'a str> {
    fn has_alphanumeric(word_w_index: &(usize, &str)) -> bool {
        word_w_index.1.chars().any(|c| c.is_alphabetic())
    }
    let word_indices: Vec<usize> = sentence
        .split_word_bound_indices()
        .filter(has_alphanumeric)
        .map(|v: (usize, &str)| v.0)
        .collect();
    let word_count_in_sentence = word_indices.len();

    let mut subtitle_list: Vec<&str> = Vec::new();
    const SUBTITLE_WORD_COUNT: usize = 10;
    const DANGLING: usize = 2; // Subtitles with 1 or 2 words are undesirable.

    let has_five_subtitles: bool = word_count_in_sentence > (4 * SUBTITLE_WORD_COUNT + DANGLING);
    let has_four_subtitles: bool = word_count_in_sentence > (3 * SUBTITLE_WORD_COUNT + DANGLING);
    let has_three_subtitles: bool = word_count_in_sentence > (2 * SUBTITLE_WORD_COUNT + DANGLING);
    let has_two_subtitles: bool = word_count_in_sentence > (SUBTITLE_WORD_COUNT + DANGLING);
    let has_one_subtitle: bool = word_count_in_sentence <= (SUBTITLE_WORD_COUNT + DANGLING);

    if has_five_subtitles {
        let subtitle1 = subtitle_from_index(0, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle2 = subtitle_from_index(1, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle3 = subtitle_from_index(2, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle4 = subtitle_from_index(3, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle5 = sentence[word_indices[4 * SUBTITLE_WORD_COUNT]..].trim();

        subtitle_list.push(subtitle1);
        subtitle_list.push(subtitle2);
        subtitle_list.push(subtitle3);
        subtitle_list.push(subtitle4);
        subtitle_list.push(subtitle5);
    } else if has_four_subtitles {
        let subtitle1 = subtitle_from_index(0, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle2 = subtitle_from_index(1, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle3 = subtitle_from_index(2, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle4 = sentence[word_indices[3 * SUBTITLE_WORD_COUNT]..].trim();

        subtitle_list.push(subtitle1);
        subtitle_list.push(subtitle2);
        subtitle_list.push(subtitle3);
        subtitle_list.push(subtitle4);
    } else if has_three_subtitles {
        let subtitle1 = subtitle_from_index(0, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle2 = subtitle_from_index(1, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle3 = sentence[word_indices[2 * SUBTITLE_WORD_COUNT]..].trim();

        subtitle_list.push(subtitle1);
        subtitle_list.push(subtitle2);
        subtitle_list.push(subtitle3);
    } else if has_two_subtitles {
        let subtitle1 = subtitle_from_index(0, SUBTITLE_WORD_COUNT, sentence, &word_indices);
        let subtitle2 = sentence[word_indices[SUBTITLE_WORD_COUNT]..].trim();

        subtitle_list.push(subtitle1);
        subtitle_list.push(subtitle2);
    } else if has_one_subtitle {
        subtitle_list.push(sentence.trim());
    }

    return subtitle_list;
}

/// Extracts a subtitle from a sentence.
///
/// Extracts a certain portion of a sentence that will form a subtitle.  Assumes
/// that each subtitle has a fixed number of words.
fn subtitle_from_index<'a>(
    subtitle_index: usize,
    subtitle_word_count: usize,
    sentence: &'a str,
    byte_indices_for_words: &Vec<usize>,
) -> &'a str {
    let first_word_index = subtitle_index * subtitle_word_count;
    let next_subtitles_first_word_index = (subtitle_index + 1) * subtitle_word_count;

    let first_byte_index = byte_indices_for_words[first_word_index];
    let next_subtitles_first_byte_index = byte_indices_for_words[next_subtitles_first_word_index];

    let subtitle = &sentence[first_byte_index..next_subtitles_first_byte_index];
    return subtitle.trim();
}
