use arbitrary::Arbitrary;

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
    let ensemble_id = &"2621346526".to_string();
    let enhanced_ensemble_id = &"2495011826".to_string();
    let compulsive_reloader_id = &"671806388".to_string();
    let enhanced_compulsive_reloader_id = &"595108252".to_string();
    let feeding_frenzy_id = &"2779035018".to_string();
    let enhanced_feeding_frenzy_id = &"1171147302".to_string();

    let flared_magwell_id = &"3230963543".to_string();
    let tactical_mag_id = &"106909392".to_string();
    let light_mag_id = &"679225683".to_string();
    let light_battery_id = &"2749775325".to_string();
    let alloy_casing_id = &"2985827016".to_string();
    let alloy_magazine_id = &"1431678320".to_string();
    let mini_frags_id = &"332133599".to_string();
    
    let fluted_barrell_id = &"1840239774".to_string();
    let counter_mass_id = &"3809316345".to_string();
    let quick_launch_id = &"3525010810".to_string();

    (perks.contains(frenzy_id) || perks.contains(enhanced_frenzy_id))
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
            || perks.contains(ensemble_id)
            || perks.contains(enhanced_ensemble_id)
            || perks.contains(compulsive_reloader_id)
            || perks.contains(enhanced_compulsive_reloader_id)
            || perks.contains(feeding_frenzy_id)
            || perks.contains(enhanced_feeding_frenzy_id)
            || perks.contains(flared_magwell_id) 
            || perks.contains(tactical_mag_id)
            || perks.contains(light_mag_id)
            || perks.contains(light_battery_id)
            || perks.contains(alloy_casing_id)
            || perks.contains(alloy_magazine_id)
            || perks.contains(mini_frags_id)
            || perks.contains(fluted_barrell_id)
            || perks.contains(counter_mass_id)
            || perks.contains(quick_launch_id)
        )
}

fn is_redundant_outlaw_combo(perks: &Vec<String>) -> bool {
    let outlaw_id = &"1168162263".to_string();
    let enhanced_outlaw_id = &"1347741687".to_string();

    let frenzy_id = &"4104185692".to_string();
    let enhanced_frenzy_id = &"3007133316".to_string();
    let rapid_hit_id = &"247725512".to_string();
    let enhanced_rapid_hit_id = &"2938480696".to_string();

    let flared_magwell_id = &"3230963543".to_string();
    let tactical_mag_id = &"106909392".to_string();
    let light_mag_id = &"679225683".to_string();
    let alloy_magazine_id = &"1431678320".to_string();
    
    (perks.contains(outlaw_id) || perks.contains(enhanced_outlaw_id))
        && (
            perks.contains(frenzy_id)
            || perks.contains(enhanced_frenzy_id)
            || perks.contains(rapid_hit_id)
            || perks.contains(enhanced_rapid_hit_id)
            || perks.contains(flared_magwell_id)
            || perks.contains(tactical_mag_id)
            || perks.contains(light_mag_id)
            || perks.contains(alloy_magazine_id)
        )
}

fn is_redundant_grenade_launcher_combo(perks: &Vec<String>) -> bool {
    // 3032599245
    let disorienting_grenades_id = &"3032599245".to_string();
    let spike_grenades_id = &"3301904089".to_string();
    let explosive_light_id = &"3194351027".to_string();
    let enhanced_explosive_light_id = &"2275087323".to_string();


    //no blast radius pls
    let confined_launch_id = &"1844523823".to_string();
    let volatile_launch_id = &"1478423395".to_string();
    let linear_compensator_id = &"1441682018".to_string();

    (perks.contains(disorienting_grenades_id) || perks.contains(spike_grenades_id) || perks.contains(explosive_light_id) || perks.contains(enhanced_explosive_light_id))
        && (
            perks.contains(confined_launch_id)
            || perks.contains(volatile_launch_id)
            || perks.contains(linear_compensator_id)
        )
}

#[derive(Clone, Arbitrary)]
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
        is_redundant_frenzy_combo(&self.perks) || is_redundant_outlaw_combo(&self.perks) || is_redundant_grenade_launcher_combo(&self.perks)
    }

    pub fn new() -> WeaponRoll {
        WeaponRoll {
            item_id: String::from(""),
            perks: Vec::from([]),
        }
    }
}
