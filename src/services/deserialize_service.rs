pub fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string
        .split(delimiter)
        .map(|value| -> String { value.trim().to_lowercase() })
        .collect();
    vec_of_strings
}

#[cfg(test)]
mod tests {
    // // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_string_exploded_by_delimiter() {
        let raw_string: &str = "123,321,456";
        let delimiter: &str = ",";
        let actual_result = explode(raw_string, delimiter);

        assert_eq!(actual_result, vec!["123", "321", "456"]);
    }

    #[test]
    fn test_whitespace_trimmed_from_strings() {
        let raw_string: &str = "123 , 321,456 ";
        let delimiter: &str = ",";
        let actual_result: Vec<String> = explode(raw_string, delimiter);

        assert_eq!(actual_result, vec!["123", "321", "456"]);
    }

    #[test]
    fn test_all_strings_lowercased() {
        let raw_string: &str = "123,HELLO,wOrLd";
        let delimiter: &str = ",";
        let actual_result: Vec<String> = explode(raw_string, delimiter);

        assert_eq!(actual_result, vec!["123", "hello", "world"]);
    }
}