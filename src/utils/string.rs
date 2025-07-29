pub trait Capitalize {
    fn capitalize(self) -> String;
}

impl<T: Into<String>> Capitalize for T {
    fn capitalize(self) -> String {
        let string = self.into();
        let mut chars = string.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
