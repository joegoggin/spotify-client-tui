pub trait ToStringVec {
    fn to_string_vec(&self) -> Vec<String>;
}

impl ToStringVec for Vec<&str> {
    fn to_string_vec(&self) -> Vec<String> {
        let mut string_vec = Vec::<String>::new();

        for str in self {
            string_vec.push(str.to_string());
        }

        string_vec
    }
}
