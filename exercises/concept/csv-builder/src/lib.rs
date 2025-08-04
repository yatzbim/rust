// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

static SPECIAL_CHARS: [char; 3] = ['\n', '\"', ','];

pub struct CsvRecordBuilder {
    content: String,
}

impl CsvRecordBuilder {
    // Create a new builder
    pub fn new() -> Self {
        CsvRecordBuilder {
            content: String::from(""),
        }
    }

    /// Adds an item to the list separated by a space and a comma.
    pub fn add(&mut self, val: &str) {
        if !self.content.is_empty() {
            self.content.push(',');
        }

        // Start string with quotes if it contains special character
        for special_char in SPECIAL_CHARS {
            if val.contains(special_char) {
                self.content.push('\"');
                break;
            }
        }

        for c in val.chars() {
            // push escape double quote if char is a double quote
            if c == '"' {
                self.content.push('\"');
            }
            self.content.push(c);
        }

        // End string with quotes if it contains special character
        for special_char in SPECIAL_CHARS {
            if val.contains(special_char) {
                self.content.push('\"');
                break;
            }
        }
    }

    /// Consumes the builder and returns the comma separated list
    pub fn build(self) -> String {
        self.content
    }
}
