use arbitrary::Arbitrary;

use crate::services::deserialize_service::explode;

fn is_bad_loose_change_combo(perks: &Vec<String>) -> bool {
    let loose_change_id = &"1119449540".to_string();
    let enhanced_loose_change_id = &"561986700".to_string();

    let permeability_id = &"2428997981".to_string();
    let enhanced_permeability_id = &"64332393".to_string();
    let voltshot_id = &"2173046394".to_string();
    let enhanced_voltshot_id = &"1720528630".to_string();
    let destabilizing_rounds_id = &"2048641572".to_string();
    let enhanced_destabilizing_rounds_id = &"1926441324".to_string();
    let chill_clip_id = &"2978966579".to_string();
    let enhanced_chill_clip_id = &"344235611".to_string();
    let incandescent_id = &"4293542123".to_string();
    let enhanced_incandescent_id = &"2675184851".to_string();
    let slice_id = &"923806249".to_string();
    let enhanced_slice_id = &"3422796781".to_string();

    (perks.contains(loose_change_id) || perks.contains(enhanced_loose_change_id))
        && !(
            perks.contains(permeability_id)
            || perks.contains(enhanced_permeability_id)
            || perks.contains(voltshot_id)
            || perks.contains(enhanced_voltshot_id)
            || perks.contains(destabilizing_rounds_id)
            || perks.contains(enhanced_destabilizing_rounds_id)
            || perks.contains(chill_clip_id)
            || perks.contains(enhanced_chill_clip_id)
            || perks.contains(incandescent_id)
            || perks.contains(enhanced_incandescent_id)
            || perks.contains(slice_id)
            || perks.contains(enhanced_slice_id)
        )
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

fn is_vorpal_weapon(perks: &Vec<String>) -> bool {
    // 3032599245
    let vorpal_weapon_id = &"1546637391".to_string();
    let enhanced_vorpal_weapon_id = &"3418165135".to_string();

    perks.contains(vorpal_weapon_id) || perks.contains(enhanced_vorpal_weapon_id)
}

fn is_shoot_to_loot(perks: &Vec<String>) -> bool {
    // 3032599245
    let shoot_to_loot_id = &"3700496672".to_string();
    let enhanced_shoot_to_loot_id = &"2840833776".to_string();

    perks.contains(shoot_to_loot_id) || perks.contains(enhanced_shoot_to_loot_id)
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
        is_redundant_frenzy_combo(&self.perks)
        || is_redundant_outlaw_combo(&self.perks)
        || is_vorpal_weapon(&self.perks)
        || is_shoot_to_loot(&self.perks)
        || is_bad_loose_change_combo(&self.perks)
    }

    pub fn new() -> WeaponRoll {
        WeaponRoll {
            item_id: String::from(""),
            perks: Vec::from([]),
        }
    }
}
