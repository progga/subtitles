//! Placeholder management.
//!
//! Placeholders are used as replacement for fixed character sequences.  These
//! are always one word e.g. PLACEHOLDER_0, PLACEHOLDER_2, etc.  This is useful,
//! among other things, to treat abbreviations.
//!
//! For example the transcript may be using "(UN)" whereas the audio narration
//! is saying "United Nations" instead of just "UN".  In these cases, the
//! placeholder can represent the longer "United Nations" as a single word.
//! Single words do not break across multiple subtitles and this is important
//! while calculating the time length of each subtitle.

use std::collections::HashMap;

/// We maintain two mappings:
/// - Abbreviation to placeholder (e.g. UN => PLACEHOLDER_0).  This is used to
///   replace abbreviations with placeholders before subtitles are prepared.
/// - Placeholder to fullform of abbreviation (e.g. PLACEHOLDER_0 => United
///   Nations).  This is used to replace placeholders with fullforms before
///   subtitle lengths are calculated.
pub struct Placeholders {
    abbr_placeholder_map: HashMap<String, String>,
    placeholder_fullform_map: HashMap<String, String>,
    placeholder_counter: u32,
}

impl Placeholders {
    /// Prepares a Placeholders object.
    pub fn new(abbr_map: &HashMap<String, String>) -> Placeholders {
        let mut placeholder = Placeholders {
            abbr_placeholder_map: HashMap::new(),
            placeholder_fullform_map: HashMap::new(),
            placeholder_counter: 0,
        };

        placeholder.generate(abbr_map);

        return placeholder;
    }

    /// Generates two hashtables.
    ///
    /// - Mapping between abbreviations and their placeholders.
    /// - Mapping between placeholders and fullforms of corresponding
    ///   abbreviations.
    fn generate(&mut self, abbr_map: &HashMap<String, String>) {
        for (abbr, fullform) in abbr_map {
            let placeholder = format!("PLACEHOLDER_{}", self.placeholder_counter);
            self.placeholder_counter += 1;

            self.abbr_placeholder_map
                .insert(abbr.to_string(), placeholder.clone());
            self.placeholder_fullform_map
                .insert(placeholder, fullform.to_string());
        }
    }

    /// Replaces abbreviations.
    ///
    /// Replaces abbreviations with corresponding placeholders.
    pub fn insert(&self, text: &str) -> String {
        let mut text_copy = text.to_string();

        for (abbr, placeholder) in self.abbr_placeholder_map.iter() {
            text_copy = text_copy.replace(abbr, placeholder);
        }

        return text_copy;
    }

    /// Replaces placeholders.
    ///
    /// Replaces placeholders with corresponding abbreviations.
    pub fn replace(&self, text: &str) -> String {
        let mut text_copy = text.to_string();

        for (abbr, placeholder) in self.abbr_placeholder_map.iter() {
            text_copy = text_copy.replace(placeholder, abbr);
        }

        return text_copy;
    }

    /// Expands placeholders.
    ///
    /// Expands placeholders with corresponding fullforms of related
    /// abbreviations.
    pub fn expand(&self, text: &str) -> String {
        let mut text_copy = text.to_string();

        for (placeholder, fullform) in self.placeholder_fullform_map.iter() {
            text_copy = text_copy.replace(placeholder, fullform);
        }

        return text_copy;
    }
}

#[cfg(test)]
#[test]
fn test_placeholder() {
    let abbr_map: HashMap<String, String> = [
        ("foo".to_string(), "bar".to_string()),
        ("baz".to_string(), "qux".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    let placeholder = Placeholders::new(&abbr_map);

    let text = "foo bar baz qux";
    let text_w_placeholders = placeholder.insert(text);
    let text_w_fullform = placeholder.expand(&text_w_placeholders);

    let expected = "bar bar qux qux";
    assert_eq!(expected, text_w_fullform);
}
