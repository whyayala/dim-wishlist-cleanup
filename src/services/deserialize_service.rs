fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string
        .split(delimiter)
        .map(|value| -> String { value.trim().to_string() })
        .collect();
    vec_of_strings
}

// pub fn deserialize_perks(string: &str) -> Vec<String> {
//     let exploded_text = explode(string, ",");
//     for item in exploded_text {
//         self.perks.push(item);
//     }
// }