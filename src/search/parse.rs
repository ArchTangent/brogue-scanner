//! Search parsing functionality for Brogue Seed Scanner.

use crate::search::*;
use crate::search::params::{PrepParams, add_parameter};

//  #######     ##     #######    ######   ########
//  ##    ##  ##  ##   ##    ##  ##        ##
//  #######  ##    ##  #######    ######   ######
//  ##       ########  ##   ##         ##  ##
//  ##       ##    ##  ##    ##  #######   ######## 

/// General-purpose parse result for all Brogue categories.
pub(crate) enum ParseResult {
    NoMatch,
    Count(CountType, u32),
    Depth(u8),
    Enchantment(i8),
    InVault(bool),
    Kind,
    Runic,
    AnyRunic,
    AllyStatus,
    LegendaryAlly,
    Mutation,
    AnyMutation,
    MagicType(MagicType),
}

/// Attempts to parse a `u32` COUNT value from a search argument.
#[inline]
fn parse_count(value: &str) -> Option<(CountType, u32)> {
    // Check if 1st char is `<` or `=`, then parse an `i8` for remaining chars.
    if value.starts_with('<') {
        match value.trim_start_matches('<').parse::<u32>() {
            Ok(c) => Some((CountType::LessThan, c)),
            Err(_) => None,
        }
    } else if value.starts_with('=') {
        match value.trim_start_matches('=').parse::<u32>() {
            Ok(c) => Some((CountType::EqualTo, c)),
            Err(_) => None,
        }
    } else {
        match value.parse::<u32>() {
            Ok(c) => Some((CountType::AtLeast, c)),
            Err(_) => None,
        }
    }
}

/// Attempts to parse a `u8` DEPTH value from a search argument.
#[inline]
fn parse_depth(value: &str) -> Option<u8> {
    // Check if 1st char is `d`, then parse a `u8` for remaining chars.
    if value.starts_with('d') {
        match value.trim_start_matches('d').parse::<u8>() {
            Ok(d) => Some(d),
            Err(_) => None,
        }
    } 
    else { 
        None 
    }
}

/// Attempts to parse a `+`/`-` `i8` ENCHANTMENT value from a search argument.
fn parse_enchantment(value: &str) -> Option<i8> {
    // Check if 1st char is `+` or `-`, then parse an `i8` for remaining chars.
    if value.starts_with('+') {
        return value.trim_start_matches('+').parse::<i8>().ok();
    } else if value.ends_with('-') {
        if let Ok(num) = value.trim_end_matches('-').parse::<i8>() {
            return Some(num * -1);
        }
    }
    None
}

/// Attempts to parse a `+` `i8` ENCHANTMENT value from a search argument.
fn parse_positive_enchantment(value: &str) -> Option<i8> {
    // Check if 1st char is `+`, then parse an `i8` for remaining chars.
    if value.starts_with('+') {
        return value.trim_start_matches('+').parse::<i8>().ok();
    }

    None
}

/// Attempts to parse a `vault`/`novault` VAULT value from a search argument.
fn parse_in_vault(value: &str) -> Option<bool> {
    if value == "vault" {
        return Some(true);   
    }
    if value == "novault" {
        return Some(false);   
    }

    None
}

/// Attempts to parse a `magic` special value from a search argument.
fn parse_magic(value: &str) -> Option<MagicType> {
    if value == "bad" {
        return Some(MagicType::Malevolent);   
    }
    if value == "good" {
        return Some(MagicType::Benevolent);   
    }

    None
}

/// Attempts to parse an altar value from a search argument.
fn parse_altar_value(value: &str) -> ParseResult {
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    if AltarKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }

    ParseResult::NoMatch
}

/// Attempts to parse an ally value from a search argument.
fn parse_ally_value(value: &str) -> ParseResult {
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }        
    // Special case with "legendary" term will look for any legendary ally.
    if value == "legendary" {
        return ParseResult::LegendaryAlly;
    }
    if AllyStatus::parse(value).is_some() {
        return ParseResult::AllyStatus;
    }    
    // Special case with "mutation" term will look for any mutation on an ally.
    if value == "mutation" {
        return ParseResult::AnyMutation;
    }
    // Partial matches (kind prioritized over mutation) 
    if MonsterKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }
    if Mutation::parse_partial(value).is_some() {
        return ParseResult::Mutation;
    }

    ParseResult::NoMatch
}

/// Attempts to parse an armor value from a search argument.
fn parse_armor_value(value: &str) -> ParseResult {
    if let Some(e) = parse_enchantment(value) {
        return ParseResult::Enchantment(e);
    }
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    // Special case with "runic" term will look for any runic armor.
    if value == "runic" {
        return ParseResult::AnyRunic;
    }
    // Partial matches (kind prioritized over runic)
    if ArmorKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }    
    if ArmorRunic::parse_partial(value).is_some() {
        return ParseResult::Runic;
    }
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }

    ParseResult::NoMatch
}

/// Attempts to parse a charm value from a search argument.
fn parse_charm_value(value: &str) -> ParseResult {
    if let Some(e) = parse_positive_enchantment(value) {
        return ParseResult::Enchantment(e);
    }
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    if CharmKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }

    ParseResult::NoMatch
}

/// Attempts to parse an equipment value from a search argument.
fn parse_equipment_value(value: &str) -> ParseResult {
    if let Some(e) = parse_enchantment(value) {
        return ParseResult::Enchantment(e);
    }
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    // Special case with "runic" term will look for any runic equipment.
    if value == "runic" {
        return ParseResult::AnyRunic;
    }
    // Partial matches
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }

    ParseResult::NoMatch
}

/// Attempts to parse a food value from a search argument.
fn parse_food_value(value: &str) -> ParseResult {
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    if FoodKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }
    ParseResult::NoMatch
}

/// Attempts to parse a gold value from a search argument.
fn parse_gold_value(value: &str) -> ParseResult {
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    ParseResult::NoMatch
}

/// Attempts to parse an item value from a search argument.
fn parse_item_value(value: &str) -> ParseResult {
    if let Some(e) = parse_enchantment(value) {
        return ParseResult::Enchantment(e);
    }
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    // Special case with "runic" term will look for any runic item.
    if value == "runic" {
        return ParseResult::AnyRunic;
    }
    // Partial matches
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }

    ParseResult::NoMatch
}

/// Attempts to parse a potion value from a search argument.
fn parse_potion_value(value: &str) -> ParseResult {
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    if PotionKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }

    ParseResult::NoMatch
}

/// Attempts to parse a ring value from a search argument.
fn parse_ring_value(value: &str) -> ParseResult {
    if let Some(e) = parse_enchantment(value) {
        return ParseResult::Enchantment(e);
    }
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    if RingKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }
    
    ParseResult::NoMatch
}

/// Attempts to parse a scroll value from a search argument.
fn parse_scroll_value(value: &str) -> ParseResult {
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    if ScrollKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }

    ParseResult::NoMatch
}

/// Attempts to parse a staff value from a search argument.
fn parse_staff_value(value: &str) -> ParseResult {
    if let Some(e) = parse_positive_enchantment(value) {
        return ParseResult::Enchantment(e);
    }
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    if StaffKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }
    
    ParseResult::NoMatch
}

/// Attempts to parse a wand value from a search argument.
fn parse_wand_value(value: &str) -> ParseResult {
    if let Some(e) = parse_positive_enchantment(value) {
        return ParseResult::Enchantment(e);
    }
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    if WandKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }
    
    ParseResult::NoMatch
}

/// Attempts to parse a weapon value from a search argument.
fn parse_weapon_value(value: &str) -> ParseResult {
    if let Some(e) = parse_enchantment(value) {
        return ParseResult::Enchantment(e);
    }
    if let Some((t, c)) = parse_count(value) {
        return ParseResult::Count(t, c);
    }
    if let Some(d) = parse_depth(value) {
        return ParseResult::Depth(d);
    }    
    // Special case: "runic" term will look for any runic weapon of given enchantment.
    if value == "runic" {
        return ParseResult::AnyRunic;
    }
    // Partial matches (kind prioritized over runic)
    if WeaponKind::parse_partial(value).is_some() {
        return ParseResult::Kind;
    }    
    if WeaponRunic::parse_partial(value).is_some() {
        return ParseResult::Runic;
    }
    if let Some(v) = parse_in_vault(value) {
        return ParseResult::InVault(v);
    }
    if let Some(m) = parse_magic(value) {
        return ParseResult::MagicType(m);
    }    

    ParseResult::NoMatch
}

/// Attempts to parse an `Ally` object from values of a search argument.
pub fn parse_allies(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() { 

        match parse_ally_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Ally, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }   
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Ally, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }                     
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Ally, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::AllyStatus => {
                if prep.ally_status.is_some() || prep.any_legendary {                    
                    add_parameter(Category::Ally, &mut prep, &mut params);
                }
                prep.ally_status = Some(value.to_owned());
            }
            ParseResult::LegendaryAlly => {
                if prep.ally_status.is_some() || prep.any_legendary {                    
                    add_parameter(Category::Ally, &mut prep, &mut params);
                }
                prep.any_legendary = true;
            }
            ParseResult::Mutation => {
                if prep.mutation.is_some() || prep.any_mutation {                    
                    add_parameter(Category::Ally, &mut prep, &mut params);
                }
                prep.mutation = Some(value.to_owned());
            }
            ParseResult::AnyMutation => {
                if prep.mutation.is_some() || prep.any_mutation {                    
                    add_parameter(Category::Ally, &mut prep, &mut params);
                }
                prep.any_mutation = true;
            }            
            _ => params.push(Err(anyhow!("'{}' is not a valid ally search term!", value))),
        }
    }
    add_parameter(Category::Ally, &mut prep, &mut params);
    
    params
}

/// Attempts to parse an `Altar` object from values of a search argument.
pub fn parse_altars(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {       

        match parse_altar_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Altar, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Altar, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Kind => {
                if prep.kind.is_some() {
                    add_parameter(Category::Altar, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            _ => params.push(Err(anyhow!("'{}' is not a valid altar search term!", value))),
        }
    }
    
    add_parameter(Category::Altar, &mut prep, &mut params);

    params
}

/// Attempts to parse an `Armor` object from values of a search argument.
pub fn parse_armors(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_armor_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Armor, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Armor, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Enchantment(new_enchantment) => {
                if prep.enchantment.is_some() {                    
                    add_parameter(Category::Armor, &mut prep, &mut params);
                }
                prep.enchantment = Some(new_enchantment);
            }
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Armor, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::Runic => {
                if prep.runic.is_some() || prep.any_runic {                    
                    add_parameter(Category::Armor, &mut prep, &mut params);
                }
                prep.runic = Some(value.to_owned());
            }
            ParseResult::AnyRunic => {
                if prep.runic.is_some() || prep.any_runic {                    
                    add_parameter(Category::Armor, &mut prep, &mut params);
                }
                prep.any_runic = true;
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Armor, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Armor, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }              
            _ => params.push(Err(anyhow!("'{}' is not a valid armor search term!", value))),
        }
    }

    add_parameter(Category::Armor, &mut prep, &mut params);
    
    params
}

/// Attempts to parse a `Charm` object from values of a search argument.
pub fn parse_charms(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_charm_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Charm, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Charm, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Enchantment(new_enchantment) => {
                if prep.enchantment.is_some() {                    
                    add_parameter(Category::Charm, &mut prep, &mut params);
                }
                prep.enchantment = Some(new_enchantment);
            }
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Charm, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Charm, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Charm, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }            
            _ => params.push(Err(anyhow!("'{}' is not a valid charm search term!", value))),
        }
    }

    add_parameter(Category::Charm, &mut prep, &mut params);

    params
}

/// Attempts to parse `Equipment` category objects from values of a search argument.
pub fn parse_equipment(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_equipment_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Equipment, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Equipment, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Enchantment(new_enchantment) => {
                if prep.enchantment.is_some() {                    
                    add_parameter(Category::Equipment, &mut prep, &mut params);
                }
                prep.enchantment = Some(new_enchantment);
            }
            ParseResult::AnyRunic => {
                if prep.runic.is_some() || prep.any_runic {                    
                    add_parameter(Category::Equipment, &mut prep, &mut params);
                }
                prep.any_runic = true;
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Equipment, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Equipment, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }            
            _ => params.push(Err(anyhow!("'{}' is not a valid equipment search term!", value))),
        }
    }

    add_parameter(Category::Equipment, &mut prep, &mut params);
    
    params
}

/// Attempts to parse a `Food` object from values of a search argument.
pub fn parse_food(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_food_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Food, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Food, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Kind => {
                if prep.kind.is_some() {
                    add_parameter(Category::Food, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            _ => params.push(Err(anyhow!("'{}' is not a valid food search term!", value))),
        }
    }

    add_parameter(Category::Food, &mut prep, &mut params);

    params
}

/// Attempts to parse a `Gold` object from values of a search argument.
pub fn parse_gold(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_gold_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Gold, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Gold, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            _ => params.push(Err(anyhow!("'{}' is not a valid gold search term!", value))),
        }
    }

    add_parameter(Category::Gold, &mut prep, &mut params);

    params
}

/// Attempts to parse `Item` category objects from values of a search argument.
pub fn parse_items(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_item_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Item, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Item, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Enchantment(new_enchantment) => {
                if prep.enchantment.is_some() {                    
                    add_parameter(Category::Item, &mut prep, &mut params);
                }
                prep.enchantment = Some(new_enchantment);
            }
            ParseResult::AnyRunic => {
                if prep.runic.is_some() || prep.any_runic {                    
                    add_parameter(Category::Item, &mut prep, &mut params);
                }
                prep.any_runic = true;
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Item, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Item, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }
            _ => params.push(Err(anyhow!("'{}' is not a valid item search term!", value))),
        }
    }

    add_parameter(Category::Item, &mut prep, &mut params);
    
    params
}

/// Attempts to parse a `Potion` object from values of a search argument.
pub fn parse_potions(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_potion_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Potion, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Potion, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Potion, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Potion, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Potion, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }
            _ => params.push(Err(anyhow!("'{}' is not a valid potion search term!", value))),
        }
    }

    add_parameter(Category::Potion, &mut prep, &mut params);

    params
}

/// Attempts to parse a `Ring` object from values of a search argument.
pub fn parse_rings(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_ring_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Ring, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Ring, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Enchantment(new_enchantment) => {
                if prep.enchantment.is_some() {                    
                    add_parameter(Category::Ring, &mut prep, &mut params);
                }
                prep.enchantment = Some(new_enchantment);
            }
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Ring, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Ring, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Ring, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }            
            _ => params.push(Err(anyhow!("'{}' is not a valid ring search term!", value))),
        }
    }

    add_parameter(Category::Ring, &mut prep, &mut params);

    params
}

/// Attempts to parse a `Scroll` object from values of a search argument.
pub fn parse_scrolls(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {
        match parse_scroll_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Scroll, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Scroll, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Scroll, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Scroll, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Scroll, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }            
            _ => params.push(Err(anyhow!("'{}' is not a valid scroll search term!", value))),
        }
    }

    add_parameter(Category::Scroll, &mut prep, &mut params);

    params
}

/// Attempts to parse a `Staff` object from values of a search argument.
pub fn parse_staves(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_staff_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Staff, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Staff, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Enchantment(new_enchantment) => {
                if prep.enchantment.is_some() {                    
                    add_parameter(Category::Staff, &mut prep, &mut params);
                }
                prep.enchantment = Some(new_enchantment);
            }
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Staff, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Staff, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Staff, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }            
            _ => params.push(Err(anyhow!("'{}' is not a valid staff search term!", value))),
        }
    }

    add_parameter(Category::Staff, &mut prep, &mut params);

    params
}

/// Attempts to parse a `Wand` object from values of a search argument.
pub fn parse_wands(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_wand_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {                    
                    add_parameter(Category::Wand, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Wand, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Enchantment(new_enchantment) => {
                if prep.enchantment.is_some() {                    
                    add_parameter(Category::Wand, &mut prep, &mut params);
                }
                prep.enchantment = Some(new_enchantment);
            }
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Wand, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Wand, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Wand, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }            
            _ => params.push(Err(anyhow!("'{}' is not a valid wand search term!", value))),
        }
    }

    add_parameter(Category::Wand, &mut prep, &mut params);

    params
}

/// Attempts to parse a `Weapon` object from values of a search argument.
pub fn parse_weapons(values: clap::Values) -> Vec<Result<ObjectParameter>> {
    let mut prep = PrepParams::default();    
    let mut params = Vec::with_capacity(1);

    for value in values.into_iter() {

        match parse_weapon_value(value) {
            ParseResult::Count(count_type, new_count) => {
                if prep.count.is_some() {
                    add_parameter(Category::Weapon, &mut prep, &mut params);
                }
                prep.count = Some(new_count);
                prep.count_type = count_type;
            }
            ParseResult::Depth(new_depth) => {
                if prep.depth.is_some() {                    
                    add_parameter(Category::Weapon, &mut prep, &mut params);
                }
                prep.depth = Some(new_depth);
            }  
            ParseResult::Enchantment(new_enchantment) => {
                if prep.enchantment.is_some() {                                        
                    add_parameter(Category::Weapon, &mut prep, &mut params);
                }
                prep.enchantment = Some(new_enchantment);
            }
            ParseResult::Kind => {
                if prep.kind.is_some() {                    
                    add_parameter(Category::Weapon, &mut prep, &mut params);
                }
                prep.kind = Some(value.to_owned());   
            }
            ParseResult::Runic => {
                if prep.runic.is_some() || prep.any_runic {                    
                    add_parameter(Category::Weapon, &mut prep, &mut params);
                }
                prep.runic = Some(value.to_owned());
            }
            ParseResult::AnyRunic => {
                if prep.runic.is_some() || prep.any_runic {                    
                    add_parameter(Category::Weapon, &mut prep, &mut params);
                }
                prep.any_runic = true;
            }
            ParseResult::InVault(in_vault) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Weapon, &mut prep, &mut params);
                }
                prep.in_vault = Some(in_vault);
            }
            ParseResult::MagicType(mtype) => {
                if prep.in_vault.is_some() {                    
                    add_parameter(Category::Weapon, &mut prep, &mut params);
                }
                prep.magic_type = Some(mtype);
            }            
            _ => params.push(Err(anyhow!("'{}' is not a valid weapon search term!", value))),
        }
    }

    add_parameter(Category::Weapon, &mut prep, &mut params);
    
    params
}
