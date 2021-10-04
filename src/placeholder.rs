//! Placeholder management.

use std::collections::HashMap;

pub struct Placeholders {
    abbr_placeholder_map: HashMap<String, String>,
    placeholder_fullform_map: HashMap<String, String>,
    placeholder_counter: u32,
}

impl Placeholders {
    /// Prepares a Placeholder object.
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
    let text_w_fullform = placeholder.replace(&text_w_placeholders);

    let expected = "bar bar qux qux";
    assert_eq!(expected, text_w_fullform);
}
