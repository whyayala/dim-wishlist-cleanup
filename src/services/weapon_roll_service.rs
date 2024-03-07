fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string
        .split(delimiter)
        .map(|value| -> String { value.trim().to_lowercase() })
        .collect();
    vec_of_strings
}

pub fn create_perks_from_text(text: &str) -> Vec<String> {
    explode(text, ",")
}

#[cfg(test)]
mod tests {
    use arbitrary::Arbitrary;
    use super::super::super::structs::weapon_roll::WeaponRoll;

    // // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_perks_from_text() {
        let bytes: [u8; 16] = [255, 40, 179, 24, 184, 113, 24, 73, 143, 51, 152, 121, 133, 143, 14, 59];
        let mut u = arbitrary::Unstructured::new(&bytes);
        let weapon_roll = WeaponRoll::arbitrary(&mut u).unwrap();
        let joined_perks = weapon_roll.perks.join(",").as_str().to_owned();
        assert_eq!(create_perks_from_text(&joined_perks), weapon_roll.perks);
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}