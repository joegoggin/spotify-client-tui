pub trait Capitalize {
    fn capitalize(self) -> String;
}

impl<T: Into<String>> Capitalize for T {
    fn capitalize(self) -> String {
        let mut capitalized = String::new();

        let s = self.into();

        if let Some(first_char) = s.chars().next() {
            capitalized.push_str(&first_char.to_uppercase().collect::<String>());
            capitalized.push_str(&s[1..]);
        } else {
            capitalized.push_str(&s);
        }

        capitalized
    }
}

#[allow(dead_code)]
pub trait StripPrefixAndSuffix {
    fn strip_prefix_and_suffix(self, prefix_and_suffix: &str) -> String;
}

impl<T: Into<String>> StripPrefixAndSuffix for T {
    fn strip_prefix_and_suffix(self, prefix_and_suffix: &str) -> String {
        let s = self.into();

        let stripped_string = s.strip_prefix(prefix_and_suffix).unwrap_or(&s);
        let stripped_string = stripped_string
            .strip_suffix(prefix_and_suffix)
            .unwrap_or(stripped_string);

        stripped_string.to_string()
    }
}
