use serde_json::Value;

pub trait GetOrDefault {
    fn get_number_or_default(&self, key: &str) -> u64;
    fn get_string_or_default(&self, key: &str) -> String;
    fn get_array_or_default(&self, key: &str) -> Vec<Value>;
    fn get_bool_or_default(&self, key: &str) -> bool;
}

impl GetOrDefault for Value {
    fn get_number_or_default(&self, key: &str) -> u64 {
        if let Some(number) = self.get(key) {
            if let Value::Number(number) = number {
                if let Some(number) = number.as_u64() {
                    return number;
                }
            }
        }

        0
    }

    fn get_string_or_default(&self, key: &str) -> String {
        if let Some(string) = self.get(key) {
            if let Value::String(string) = string {
                return string.to_owned();
            }
        }

        "".to_string()
    }

    fn get_array_or_default(&self, key: &str) -> Vec<Value> {
        if let Some(array) = self.get(key) {
            if let Value::Array(array) = array {
                return array.to_owned();
            }
        }

        vec![]
    }

    fn get_bool_or_default(&self, key: &str) -> bool {
        if let Some(bool) = self.get(key) {
            if let Value::Bool(bool) = bool {
                return bool.to_owned();
            }
        }

        false
    }
}
