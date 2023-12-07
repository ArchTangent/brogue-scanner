//! Search structs and functionality parameters for Brogue Seed Scanner.

mod params;
mod parse;

pub use params::SearchParameters;
use crate::objects::{
    Category, Object, MagicType, AllyStatus, AltarKind, ArmorKind, ArmorRunic, 
    CharmKind, FoodKind, GoldKind, KeyKind, MonsterKind, Mutation, PotionKind, 
    RingKind, StaffKind, ScrollKind, WandKind, WeaponKind, WeaponRunic
};
use crate::file_handling::FileFormat;
use anyhow::{anyhow, Result};
use csv::{ReaderBuilder, StringRecord};
use encoding_rs_io::DecodeReaderBytesBuilder;
use params::ObjectParameter;
use std::fs::File;
use std::io::Read;

/// Whether or not a search is fully complete (max # of search results met).
#[repr(u8)]
pub(crate) enum SearchStatus {
    InProgress,
    /// The entire search is complete - stop the search
    EndOfSearch,
    /// All object parameters met for a seed - move to next seed
    AllObjectsFound,
    /// Early exit - for "less than" and "exact" params.  Advanced seed immediately.
    EarlySeedExit,
    /// End of the file (EOF) has been reached
    EndOfFile,
}

/// Match Count type for object parameters fields, with "At Least" being default.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub(crate) enum CountType {
    /// Object match count should be ">=" object match target.
    AtLeast,
    /// Object match count should be "<" object match target.
    LessThan,
    /// Object match count should be "=" object match target.
    EqualTo,
}

impl Default for CountType {
    fn default() -> Self {
        CountType::AtLeast
    }
}

/// How search parameters should respond to a given match, beased on the count type
/// of the object parameters matched.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MatchResponse {
    /// Increment object match counter
    Increment,
    /// Take no action
    DoNothing,
    /// Trigger an early exit (LessThan / EqualTo conditions exceeded)
    EarlyExit,
}

/// Prints all `SearchMatch` instances.
/// - Verbosity  1: displays only seed with matches
/// - Verbosity  2: displays seed and depth with matches
/// - Verbosity  3: displays seed, depth, and items in each match
// pub fn display_matches(matches: &Vec<SearchMatch>, verbosity: u8) {
pub fn display_matches(matches: &Vec<SearchMatch>, params: &SearchParameters) {
    let mut seed_count = 0;
    let mut seed = 0;
    let mut depth = 0;

    if matches.len() > 0 {
        println!("Matches:\n");
    }
    
    for m in matches {
        if m.seed != seed {
            seed = m.seed;
            depth = 0;
            seed_count += 1;
            println!("Seed {}", seed);
        }
        if m.depth != depth && params.verbosity > 1 {
            depth = m.depth;
            println!("    Depth {}", depth);
        }
        if params.verbosity > 2 {
            println!("        {}", m);
        }
    }
    println!("\n...{} matches found.\n", seed_count);
}

/// Holds a matching search results for a query.
#[derive(Debug, Clone)]
pub struct SearchMatch {
    /// Whether a match resulted in success or failure (MatchType::LessThan / EqualTo)
    pub match_resp: MatchResponse,    
    // Object Data
    pub seed: u32,
    pub depth: u8,
    pub object: Object,
    /// Vault in which object is held
    pub vault: Option<u8>,
    /// Monster holding the object
    pub carried_by: Option<MonsterKind>,
}

impl SearchMatch {
    /// Creates a new instance from a CSV Record.  `unwrap()` is used because values
    /// are known to be present.
    pub(crate) fn from_record(
        category: Category,
        match_resp: MatchResponse, 
        seed: u32,
        depth: u8, 
        record: &StringRecord
    ) -> Self {        
        use Category::*;

        let object = match category {
            Weapon => {
                let kind = WeaponKind::parse(&record[5]).unwrap();
                let enchantment = record[6].parse::<i8>().unwrap();
                let runic = WeaponRunic::parse(&record[7]);
                Object::new_weapon(kind, enchantment, runic)
            }
            Armor => {
                let kind = ArmorKind::parse(&record[5]).unwrap();
                let enchantment = record[6].parse::<i8>().unwrap();
                let runic = ArmorRunic::parse(&record[7]);
                Object::new_armor(kind, enchantment, runic)
            }
            Potion => {
                let kind = PotionKind::parse(&record[5]).unwrap();
                Object::new_potion(kind)
            }            
            Scroll => {
                let kind = ScrollKind::parse(&record[5]).unwrap();
                Object::new_scroll(kind)
            }
            Charm => {
                let kind = CharmKind::parse(&record[5]).unwrap();
                let enchantment = record[6].parse::<i8>().unwrap();
                Object::new_charm(kind, enchantment)
            }
            Ring => {
                let kind = RingKind::parse(&record[5]).unwrap();
                let enchantment = record[6].parse::<i8>().unwrap();
                Object::new_ring(kind, enchantment)
            }
            Staff => {
                let kind = StaffKind::parse(&record[5]).unwrap();
                let enchantment = record[6].parse::<i8>().unwrap();
                Object::new_staff(kind, enchantment)
            }
            Wand => {
                let kind = WandKind::parse(&record[5]).unwrap();
                let enchantment = record[6].parse::<i8>().unwrap();
                Object::new_wand(kind, enchantment)
            }   
            Ally => {
                let kind = MonsterKind::parse(&record[5]).unwrap();
                let status = AllyStatus::parse(&record[11]).unwrap();
                let mutation = Mutation::parse(&record[12]);
                Object::new_ally(kind, status, mutation)
            }
            Food => {
                let kind = FoodKind::parse(&record[5]).unwrap();
                Object::new_food(kind)
            }
            Gold => {
                let kind = GoldKind::parse(&record[5]).unwrap();
                let count = record[3].parse::<u32>().unwrap();
                Object::new_gold(kind, count)
            }
            Altar => {
                let kind = AltarKind::parse(&record[5]).unwrap();
                Object::new_altar(kind)
            }
            Key => {
                let kind = KeyKind::parse(&record[5]).unwrap();
                let opens = record[9].parse::<u8>().ok();
                Object::new_key(kind, opens)
            }
            // Items and Equipment can't be created from csv Records
            _ => unreachable!(),
        };

        let vault = record[8].parse::<u8>().ok();
        let carried_by = MonsterKind::parse(&record[10]);

        Self {
            match_resp,
            seed,
            depth,
            object,
            vault,
            carried_by,
        }
    }
}

impl std::fmt::Display for SearchMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(monster) = self.carried_by {
            return write!(f, "{} ({})", self.object, monster);
        }
        if let Some(vault) = self.vault {
            return write!(f, "{} (vault {})", self.object, vault);
        }
        write!(f, "{}", self.object)
    }
}

/// Searches filepaths specified using given `SearchParameter`s, and 
/// returns a list of `SearchResult`s based on matches and level of detail (LOD).
pub fn search_files(
    search: &mut SearchParameters,
) -> Result<Vec<SearchMatch>> {
    // Always display the search information for user feedback
    println!("{}", search);

    if search.file_paths.is_empty() {
        return Err(anyhow!("No files found!"));
    }

    let mut results = Vec::with_capacity(search.search_match_target.into());
    let file_paths = search.file_paths.clone();

    match search.format {
        FileFormat::Utf8 => {
            for file_path in file_paths.iter() {
                if search.debug {
                    println!("searching file: {:?}", file_path);
                }                        
                let file = File::open(file_path)?;

                match search_file(file, search, &mut results) {
                    Ok(SearchStatus::EndOfSearch) => return Ok(results),
                    _ => (),
                }
            }   
        }
        FileFormat::Utf16 => {
            for file_path in file_paths.iter() {
                if search.debug {
                    println!("searching file: {:?}", file_path);
                }                
                let file = File::open(file_path)?;
                let new_file = DecodeReaderBytesBuilder::new()
                    .encoding(Some(encoding_rs::UTF_16LE))
                    .build(file);

                match search_file(new_file, search, &mut results) {
                    Ok(SearchStatus::EndOfSearch) => return Ok(results),
                    _ => (),
                }
            }
        }
    }

    Ok(results)
}

/// Searches specified filepath using given search parameters, and passes results
/// into given list of search results.  If `find_all` is `true`, the seed will continue 
/// to be explored even after ObjectParameters have been satisfied.
fn search_file<F: Read>(
    file: F,
    search: &mut SearchParameters,
    results: &mut Vec<SearchMatch>,
) -> Result<SearchStatus> {
    use SearchStatus::*;

    let depth_min = search.depth_min;
    let depth_max = search.depth_max;
    let mut next_seed = search.seed_min;
    let mut temp = Vec::with_capacity(10);
    let mut prev_seed = 0;
    // Flag for AllObjectsFound condition.
    let mut all_object_flag = false;

    let mut rdr = ReaderBuilder::new()
        .from_reader(file);

    {
        // Validate header
        let headers = rdr.headers()?;
        if !(headers.len() == 13)
            || !headers.as_slice().contains("dungeon_versionseeddepth") 
        {
            return Err(anyhow!("Invalid Brogue csv header"));
        }
    }

    // Clear any search data from a previous file (as it's a new seed)
    search.clear();

    // Validate then search 1st line
    if let Some(result) = rdr.records().next() {
        let record = result?;  

        // Early exit if 1st line is OOB (e.g. seed > seed_max)
        let (in_bounds, seed, depth) = bounds_check(
            &record, next_seed, search.seed_max, depth_min, depth_max
        )?;

        prev_seed = seed;

        if in_bounds {
            if let Some(search_match) = search_record(seed, depth, &record, search)? {
                let status = search.search_status(search_match.match_resp);
                temp.push(search_match);     
                
                match status {
                    AllObjectsFound => {
                        all_object_flag = true
                    }
                    EarlySeedExit => {
                        next_seed += 1;
                        all_object_flag = false;
                    }
                    _ => (),
                }
            }
        } else {
            return Ok(EndOfFile);   
        }
    }

    // Search remaining lines in the file
    for record_result in rdr.records() {
        let record = record_result?;
       
        let (in_bounds, seed, depth) = bounds_check(
            &record, next_seed, search.seed_max, depth_min, depth_max
        )?;

        // Clear the temp buffer, search and object counters on new seed
        if seed != prev_seed {
            if all_object_flag && search.is_valid() {
                results.extend_from_slice(&temp);
                search.search_matches += 1;
                all_object_flag = false;

                if search.is_complete() {
                    break;
                }
            }
            all_object_flag = false;
            search.clear();
            temp.clear();
        }

        prev_seed = seed;

        if in_bounds {
            if let Some(search_match) = search_record(seed, depth, &record, search)? {
                let status = search.search_status(search_match.match_resp);
                temp.push(search_match);           

                match status {
                    AllObjectsFound =>{
                        all_object_flag = true;
                    } 
                    EarlySeedExit => {
                        next_seed += 1;
                        all_object_flag = false;
                    }
                    _ => (),
                }
            }
        }       
    }

    // Final status check at end of file (in case of matches on final seed in file).
    if all_object_flag && search.is_valid() {
        results.extend_from_slice(&temp);
        search.search_matches += 1;  
    }

    match search.is_complete() {
        false => Ok(EndOfFile),
        true => Ok(EndOfSearch)
    }
}

/// Searches specified Record (line in .csv file) using given search parameters, and 
/// passes results into given list of search results.  Assumes that CSVs are in proper
/// format, and as such uses `unwrap` on each Record's fields.
fn search_record(
    seed: u32,
    depth: u8,
    record: &StringRecord,
    search: &mut SearchParameters,
) -> Result<Option<SearchMatch>> {
    let category = Category::parse(&record[4]).unwrap();
    let category_flags = category.to_flags();
  
    // Return the first matching SearchResult (at most one per Record)
    for param in search.object_params.iter_mut() {
        if category_flags.intersects(param.category_flags) && depth <= param.depth {
            if let Some(result) = search_category(seed, depth, param.category, &record, param)? {                
                return Ok(Some(result));
            }
        } 
    }

    Ok(None)
}

/// Searches specified Record (line in .csv file) for a given Category.  If a match,
/// updates search results. Assumes that CSVs are in proper format, and as such uses 
/// `unwrap` on each Record's fields.
fn search_category(
    seed: u32,
    depth: u8,
    param_category: Category,
    record: &StringRecord,
    param: &mut ObjectParameter,
) -> Result<Option<SearchMatch>> {
    use Category::*;

    let mut matched = true;
    let record_category = Category::parse(&record[4]).unwrap();

    match param_category {
        Weapon | Armor => {
            if let Some(kind) = param.kind.as_ref() {
                matched &= record[5].contains(kind);
            }
            if let Some(enchantment) = param.enchantment {
                let rec_enchantment = record[6].parse::<i8>()?;

                if enchantment >= 0 {
                    matched &= rec_enchantment >= enchantment;
                } else {
                    matched &= rec_enchantment <= enchantment;
                }
            }
            if param.any_runic {
                matched &= !&record[7].is_empty();
            } else if let Some(runic) = param.runic.as_ref() {
                matched &= record[7].contains(runic);
            }
            if let Some(in_vault) = param.in_vault.as_ref() {
                matched &= match (in_vault, record[8].is_empty()) {
                    (true, true) => false,
                    (true, false) => true,
                    (false, true) => true,
                    (false, false) => false,
                }
            }
            if let Some(magic_type) = param.magic_type.as_ref() {
                matched &= magic_check(record_category, *magic_type, record)
            }            
        }
        Charm | Ring | Staff | Wand => {
            if let Some(kind) = param.kind.as_ref() {
                matched &= record[5].contains(kind);
            }
            if let Some(enchantment) = param.enchantment {
                let rec_enchantment = record[6].parse::<i8>()?;

                if enchantment >= 0 {
                    matched &= rec_enchantment >= enchantment;
                } else {
                    matched &= rec_enchantment <= enchantment;
                }
            }
            if let Some(in_vault) = param.in_vault.as_ref() {
                matched &= match (in_vault, record[8].is_empty()) {
                    (true, true) => false,
                    (true, false) => true,
                    (false, true) => true,
                    (false, false) => false,
                }
            }
            if let Some(magic_type) = param.magic_type.as_ref() {
                matched &= magic_check(record_category, *magic_type, record)
            }                      
        }
        Potion | Scroll => {
            if let Some(kind) = param.kind.as_ref() {
                matched &= record[5].contains(kind);
            }
            if let Some(in_vault) = param.in_vault.as_ref() {
                matched &= match (in_vault, record[8].is_empty()) {
                    (true, true) => false,
                    (true, false) => true,
                    (false, true) => true,
                    (false, false) => false,
                }
            }
            if let Some(magic_type) = param.magic_type.as_ref() {
                matched &= magic_check(record_category, *magic_type, record)
            }               
        }
        Food | Altar => {
            if let Some(kind) = param.kind.as_ref() {
                matched &= record[5].contains(kind);
            }
        }
        Ally => {
            if let Some(kind) = param.kind.as_ref() {
                matched &= record[5].contains(kind);
            }
            if param.any_legendary {
                matched &= &record[11] == "allied";
            } else if let Some(ally_status) = param.ally_status.as_ref() {
                matched &= ally_status == &record[11];
            }    
            if param.any_mutation {
                matched &= !&record[12].is_empty();
            } else if let Some(mutation) = param.mutation.as_ref() {
                matched &= record[12].contains(mutation);
            }                        
        }
        Equipment | Item => {
            if let Some(enchantment) = param.enchantment {
                match record_category {
                    Armor | Charm | Ring | Staff | Wand | Weapon => {
                        let rec_enchantment = record[6].parse::<i8>()?;

                        if enchantment >= 0 {
                            matched &= rec_enchantment >= enchantment;
                        } else {
                            matched &= rec_enchantment <= enchantment;
                        }
                    }
                    _ => matched = false
                }                
            }   
            if param.any_runic {
                match record_category {
                    Armor | Weapon => {
                        matched &= !&record[7].is_empty();
                    }
                    _ => matched = false
                }                      
                
            }                
            if let Some(in_vault) = param.in_vault.as_ref() {
                matched &= match (in_vault, record[8].is_empty()) {
                    (true, true) => false,
                    (true, false) => true,
                    (false, true) => true,
                    (false, false) => false,
                }
            }                 
            if let Some(magic_type) = param.magic_type.as_ref() {
                matched &= magic_check(record_category, *magic_type, record)
            }               
        }
        // Key and Gold don't have any specific parameters to check aside from COUNT
        _ => (),
    }

    // If a successful match, add SearchResult for given seed and depth
    if matched {
        let count = record[3].parse::<u32>()?;
        param.count += count;
        let pc = param.count;
        let pc_tgt = param.count_target; 

        // NOTE: 'DoNothing' matches still added, but don't count toward 'count target'.
        // 'AtLeast'  - increments unless count > count target, never exits
        // 'LessThan' - early exits if >= count target, else do nothing
        // 'EqualTo'  - early exits if > count target, else do nothing
        let match_type = match (param.count_type, pc < pc_tgt, pc > pc_tgt) {
            (CountType::AtLeast, _, false) => MatchResponse::Increment,
            (CountType::LessThan, true, _) => MatchResponse::Increment,
            (CountType::LessThan, false, _) => MatchResponse::EarlyExit,
            (CountType::EqualTo, _, false) => MatchResponse::Increment,
            (CountType::EqualTo, _, true) => MatchResponse::EarlyExit,            
            _ => MatchResponse::DoNothing,
        };

        return Ok(Some(
            SearchMatch::from_record(record_category, match_type, seed, depth, &record))
        );   
    }

    Ok(None)
}

/// Helper function to filter a CSV record by seed and depth.
#[inline]
fn bounds_check(r: &StringRecord, s1: u32, s2: u32, d1: u8, d2: u8) -> Result<(bool, u32, u8)> {
    let seed = r[1].parse::<u32>()?;        
    let depth = r[2].parse::<u8>()?;   
    let in_bounds = seed >= s1 
        && seed <= s2 
        && depth >= d1 
        && depth <= d2;

    Ok((in_bounds, seed, depth))   
}

/// Returns true if the object's `MagicType` (benevolent/malevolent) matches.
#[inline]
fn magic_check(
    record_category: Category, 
    magic_type: MagicType,  
    record: &StringRecord,
) -> bool {  
    use Category::*;
    use MagicType::*;
    let enchantment = record[6].parse::<i8>().ok();

    match record_category {
        Armor | Charm | Ring | Weapon => {
            match (magic_type, enchantment) {
                (Benevolent, Some(e)) => e > 0,
                (Malevolent, Some(e)) => e < 0,
                _ => true,
            }
        }
        Potion => {
            let is_malevolent = PotionKind::parse(&record[5]).unwrap().is_malevolent();
            match (magic_type, is_malevolent) {
                (Malevolent, true) => true,
                (Benevolent, false) => true,
                _ => false,
            }
        }
        Scroll => {
            let is_malevolent = ScrollKind::parse(&record[5]).unwrap().is_malevolent();
            match (magic_type, is_malevolent) {
                (Malevolent, true) => true,
                (Benevolent, false) => true,
                _ => false,
            }
        }
        Staff => {
            let is_malevolent = StaffKind::parse(&record[5]).unwrap().is_malevolent();
            match (magic_type, is_malevolent) {
                (Malevolent, true) => true,
                (Benevolent, false) => true,
                _ => false,
            }
        }
        Wand => {
            let is_malevolent = WandKind::parse(&record[5]).unwrap().is_malevolent();
            match (magic_type, is_malevolent) {
                (Malevolent, true) => true,
                (Benevolent, false) => true,
                _ => false,
            }
        }
        // Ally, Altar, Food, Gold, Key aren't magical, and Records can't be Item/Equipment
        _ => false,
    }
}
