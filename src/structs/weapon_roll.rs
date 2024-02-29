fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string
        .split(delimiter)
        .map(|value| -> String { value.trim().to_lowercase() })
        .collect();
    vec_of_strings
}

fn filter_redundant_frenzy_combos(weapon_rolls: Vec<WeaponRoll>) -> Vec<WeaponRoll> {
    let filtered_weapon_rolls = weapon_rolls
        .into_iter()
        .filter(|weapon_roll| is_redundant_frenzy_combo(&weapon_roll.perks))
        .collect();
    filtered_weapon_rolls
}

fn is_redundant_frenzy_combo(perks: &Vec<String>) -> bool {
    let frenzy_id = &"4104185692".to_string();
    let surplus_id = &"3436462433".to_string();
    let flared_magwell_id = &"3230963543".to_string();
    let tactical_mag_id = &"106909392".to_string();
    perks.contains(frenzy_id) 
        && (perks.contains(surplus_id) || perks.contains(flared_magwell_id) || perks.contains(tactical_mag_id))

}

#[derive(Clone)]
pub struct WeaponRoll {
    pub item_id: String,
    pub perks: Vec<String>,
}

impl WeaponRoll {
    pub fn add_perks_from_text(&mut self, text: &str) {
        let exploded_text = explode(text, ",");
        for item in exploded_text {
            self.perks.push(item);
        }
    }

    pub fn is_bad_perk_combo(&mut self) -> bool {
        is_redundant_frenzy_combo(&self.perks)
    }

    pub fn new() -> WeaponRoll {
        WeaponRoll {
            item_id: String::from(""),
            perks: Vec::from([]),
        }
    }
}
