//! Prepares subtitles from given text.

use unicode_segmentation::UnicodeSegmentation; // 1.7.1

/// Split the entire file into *potential* subtitles.
pub fn split_into_subtitles(file_content: &str) -> Vec<String> {
    let mut subtitle_list: Vec<String> = Vec::new();
    let sentence_list: Vec<&str> = file_content.unicode_sentences().collect();

    for sentence in sentence_list {
        let mut subtitles_for_a_sentence = prepare_subtitles_from_sentence(sentence, 10, 4);
        subtitle_list.append(&mut subtitles_for_a_sentence);
    }

    return subtitle_list;
}

#[cfg(test)]
#[test]
/// Tests prepare_subtitles_from_sentence().
fn test_prepare_subtitles_from_sentence() {
    // Dangling subtitle should be merged with second last one.
    let long_sentence = "Jerusalem is a city in Western Asia, on a plateau in the Judaean Mountains between the Mediterranean and the Dead Sea.";
    let subtitle_list = prepare_subtitles_from_sentence(long_sentence, 9, 4);

    let expected = vec![
        "Jerusalem is a city in Western Asia, on a".to_string(),
        "plateau in the Judaean Mountains between the Mediterranean and the Dead Sea.".to_string(),
    ];
    assert_eq!(expected, subtitle_list);

    // Three subtitles.
    let subtitle_list2 = prepare_subtitles_from_sentence(long_sentence, 8, 4);

    let expected2 = vec![
        "Jerusalem is a city in Western Asia, on".to_string(),
        "a plateau in the Judaean Mountains between the".to_string(),
        "Mediterranean and the Dead Sea.".to_string(),
    ];
    assert_eq!(expected2, subtitle_list2);

    // Three subtitles; last one is the smallest possible.
    let sentence_w_20_words = "Jerusalem is a city in Western Asia, on a plateau in the Judaean Mountains between the Mediterranean and the Dead...";
    let subtitle_list3 = prepare_subtitles_from_sentence(sentence_w_20_words, 8, 4);

    let expected3 = vec![
        "Jerusalem is a city in Western Asia, on".to_string(),
        "a plateau in the Judaean Mountains between the".to_string(),
        "Mediterranean and the Dead...".to_string(),
    ];
    assert_eq!(expected3, subtitle_list3);

    // No dandling subtitle is possible.
    let sentence_w_18_words = "Jerusalem is a city in Western Asia, on a plateau in the Judaean Mountains between the Mediterranean and...";
    let subtitle_list4 = prepare_subtitles_from_sentence(sentence_w_18_words, 9, 4);

    let expected4 = vec![
        "Jerusalem is a city in Western Asia, on a".to_string(),
        "plateau in the Judaean Mountains between the Mediterranean and...".to_string(),
    ];
    assert_eq!(expected4, subtitle_list4);
}

/// Split a sentence into subtitles.
fn prepare_subtitles_from_sentence<'a>(
    sentence: &'a str,
    subtitle_max_word_count: usize,
    subtitle_min_word_count: usize,
) -> Vec<String> {
    fn has_alphanumeric(word_w_index: &(usize, &str)) -> bool {
        word_w_index.1.chars().any(|c| c.is_alphabetic())
    }

    let word_indices: Vec<usize> = sentence
        .split_word_bound_indices()
        .filter(has_alphanumeric)
        .map(|v: (usize, &str)| v.0)
        .collect();
    let word_count_in_sentence = word_indices.len();

    let mut subtitle_list: Vec<String> = Vec::new();

    let mut cumulative_word_count = 0;
    let mut subtitle_count = 0;
    while (cumulative_word_count + subtitle_max_word_count) < word_count_in_sentence {
        let subtitle = subtitle_from_index(
            subtitle_count,
            subtitle_max_word_count,
            sentence,
            &word_indices,
        );
        subtitle_list.push(subtitle.to_string());

        cumulative_word_count += subtitle_max_word_count;
        subtitle_count += 1;
    }

    // Any remaining subtitle no longer than subtitle_max_word_count.
    let last_subtitle = sentence[word_indices[subtitle_count * subtitle_max_word_count]..].trim();
    subtitle_list.push(last_subtitle.to_string());

    let last_subtitle_word_count = word_count_in_sentence % subtitle_max_word_count;
    if last_subtitle_word_count != 0 && last_subtitle_word_count < subtitle_min_word_count {
        adjust_last_subtitle(&mut subtitle_list);
    }

    return subtitle_list;
}

#[cfg(test)]
#[test]
/// Tests adjust_last_subtitle().
fn test_adjust_last_subtitle() {
    // Only one subtitle.
    let mut subtitle_list = vec!["foo".to_string()];
    adjust_last_subtitle(&mut subtitle_list);

    let expected = vec!["foo".to_string()];
    assert_eq!(expected, subtitle_list);

    // Several subtitles.
    let mut subtitle_list2 = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
    adjust_last_subtitle(&mut subtitle_list2);

    let expected2 = vec!["foo".to_string(), "bar baz".to_string()];
    assert_eq!(expected2, subtitle_list2);
}

/// Join last two subtitles.
fn adjust_last_subtitle(subtitle_list: &mut Vec<String>) {
    if subtitle_list.len() < 2 {
        return;
    }

    let last = subtitle_list.pop().unwrap();
    let second_last = subtitle_list.pop().unwrap();

    let new_last = format!("{} {}", second_last, last);
    subtitle_list.push(new_last);
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
