fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string
        .split(delimiter)
        .map(|value| -> String { value.trim().to_lowercase() })
        .collect();
    vec_of_strings
}

fn is_redundant_frenzy_combo(perks: &Vec<String>) -> bool {
    let frenzy_id = &"4104185692".to_string();
    let enhanced_frenzy_id = &"3007133316".to_string();
    
    let surplus_id = &"3436462433".to_string();
    let enhanced_surplus_id = &"2658083589".to_string();
    let outlaw_id = &"1168162263".to_string();
    let enhanced_outlaw_id = &"1347741687".to_string();
    let perpetual_motion_id = &"1428297954".to_string();
    let enhanced_perpetual_motion_id = &"2014892510".to_string();
    let enlightened_acton_id = &"3828510309".to_string();
    let enhanced_enlightened_acton_id = &"1771736209".to_string();
    let rapid_hit_id = &"247725512".to_string();
    let enhanced_rapid_hit_id = &"2938480696".to_string();
    let threat_detector_id = &"4071163871".to_string();
    let enhanced_threat_detector_id = &"494941759".to_string();

    let flared_magwell_id = &"3230963543".to_string();
    let tactical_mag_id = &"106909392".to_string();
    let light_mag_id = &"679225683".to_string();
    
    let fluted_barrell_id = &"1840239774".to_string();
    perks.contains(frenzy_id) || perks.contains(enhanced_frenzy_id)
        && (
            perks.contains(surplus_id) 
            || perks.contains(enhanced_surplus_id) 
            || perks.contains(outlaw_id)
            || perks.contains(enhanced_outlaw_id)
            || perks.contains(perpetual_motion_id)
            || perks.contains(enhanced_perpetual_motion_id)
            || perks.contains(enlightened_acton_id)
            || perks.contains(enhanced_enlightened_acton_id)
            || perks.contains(rapid_hit_id)
            || perks.contains(enhanced_rapid_hit_id)
            || perks.contains(threat_detector_id)
            || perks.contains(enhanced_threat_detector_id)
            || perks.contains(flared_magwell_id) 
            || perks.contains(tactical_mag_id)
            || perks.contains(light_mag_id)
            || perks.contains(fluted_barrell_id)
        )

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
