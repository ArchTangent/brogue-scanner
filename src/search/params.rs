//! Search parameters for Brogue Seed Scanner.

use anyhow::{anyhow, Result};
use crate::bitflags::BitFlags16;
use crate::file_handling::{get_brogue_csv_paths, FileFormat};
use crate::objects::{Category, MagicType};
use crate::search::{SearchStatus, CountType, MatchResponse};
use crate::search::parse::*;
use std::env::current_dir;
use std::path::{Path, PathBuf};

/// Specific search parameter for an object category (armor, weapon, etc.).
/// Checked against each line of a csv record.
#[derive(Debug, PartialEq)]
pub struct ObjectParameter {
    /// Current count matched for the active seed
    pub(crate) count: u32,
    /// Minimum number of times the parameter must match per seed.  Defaults to 1.
    pub(crate) count_target: u32,
    /// How object count should compare to object count target for successful match.
    pub(crate) count_type: CountType,
    /// Object category to be matched against the csv record.
    pub(crate) category: Category,
    /// Bitflag representation of category (can have more than 1)
    pub(crate) category_flags: BitFlags16,  
    /// Object kind matched against record.
    pub(crate) kind: Option<String>,
    /// Maximum depth at which to search for object (specific to this object)
    pub(crate) depth: u8,      
    /// Enchantment level.
    pub(crate) enchantment: Option<i8>,
    /// Weapon or Armor runic.
    pub(crate) runic: Option<String>,
    /// Special case where any (non-empty) runic is valid - when "runic" term used.
    pub(crate) any_runic: bool,
    /// Ally status.
    pub(crate) ally_status: Option<String>,
    /// Special case for legendary allies - when "legendary" term is used.
    pub(crate) any_legendary: bool,
    /// Ally mutation.
    pub(crate) mutation: Option<String>,
    /// Special case for any mutation - when "mutation" term is used.
    pub(crate) any_mutation: bool,
    /// Whether item is in a vault (for items that _can_ be in a vault).
    pub(crate) in_vault: Option<bool>,
    /// Whether Potion / Scroll / Staff / Wand is benevolent or malevolent.
    pub(crate) magic_type: Option<MagicType>,    
}

impl ObjectParameter {
    /// Makes a new search parameter from a `PrepParams` struct.
    pub fn from_prep(category: Category, prep: &mut PrepParams) -> Self {
        Self {
            count: 0,
            count_target: prep.count.unwrap_or(1),
            count_type: prep.count_type,
            category,
            category_flags: category.to_flags(),
            kind: prep.kind.take(),
            depth: prep.depth.unwrap_or(40),
            enchantment: prep.enchantment,
            runic: prep.runic.take(),
            any_runic: prep.any_runic,
            ally_status: prep.ally_status.take(),
            any_legendary: prep.any_legendary,
            mutation: prep.mutation.take(),
            any_mutation: prep.any_mutation,
            in_vault: prep.in_vault.take(),
            magic_type: prep.magic_type.take(),
        }
    }
    /// Clears `count` field.
    pub fn clear(&mut self) {
        self.count = 0;
    }
    /// Returns `true` if and ObjectParameters is valid based on `CountType`:
    /// - AtLeast:   count > count_target
    /// - EqualTo:   count == count_target
    /// - LessThan:  count < count_target
    pub(crate) fn is_valid(&self) -> bool {
        match self.count_type {
            CountType::AtLeast => self.count >= self.count_target,
            CountType::LessThan => self.count < self.count_target,
            CountType::EqualTo => self.count == self.count_target,
        }
    }    
}

impl std::fmt::Display for ObjectParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CountType::*;

        write!(f, "  category: {}\n", self.category)?;

        match self.count_type {
            AtLeast => write!(f, "     count: {} or more\n", self.count_target)?,
            LessThan => write!(f, "     count: less than {}", self.count_target)?,
            EqualTo => write!(f, "     count: exactly {}\n", self.count_target)?,
        };       
        match self.depth {
            26 | 40 => (),
            _ => write!(f, "     depth: {} or less\n", self.depth)?,
        };   
        if let Some(kind) = self.kind.as_ref() {
            write!(f, "      kind: {}\n", kind)?;
        }
        if let Some(enchantment) = self.enchantment.as_ref() {
            write!(f, "      ench: {}\n", enchantment)?;
        }
        if let Some(runic) = self.runic.as_ref() {
            write!(f, "     runic: {}\n", runic)?;
        }        
        if self.any_runic {
            write!(f, "     runic: any\n")?;
        }
        if let Some(ally_status) = self.ally_status.as_ref() {
            write!(f, "    status: {}\n", ally_status)?;
        }     
        if self.any_legendary {
            write!(f, "    status: legendary\n")?;
        }
        if let Some(mutation) = self.mutation.as_ref() {
            write!(f, "  mutation: {}\n", mutation)?;
        }         
        if self.any_mutation {
            write!(f, "  mutation: any\n")?;
        }

        Ok(())
    }
}

/// Values used to prepare a `SearchParameters` struct.
#[derive(Default, PartialEq)]
pub struct PrepParams {
    pub(crate) kind: Option<String>,
    pub(crate) count: Option<u32>,
    pub(crate) count_type: CountType,
    pub(crate) depth: Option<u8>,  
    pub(crate) enchantment: Option<i8>,
    pub(crate) runic: Option<String>,
    pub(crate) any_runic: bool,
    pub(crate) ally_status: Option<String>,
    pub(crate) any_legendary : bool,
    pub(crate) mutation: Option<String>,
    pub(crate) any_mutation: bool,
    pub(crate) in_vault: Option<bool>,
    pub(crate) magic_type: Option<MagicType>,          
}

impl PrepParams {
    /// Makes a new instance.
    pub fn new() -> Self {
        Self::default()
    }
    /// Checks if struct is empty:  equal to default struct.
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

/// Contains all possible parameters used for a Brogue seed search, including:
/// - General:  depth_min, depth_max, detail, etc.
/// - Object:  parameters for a given object category (armor, weapon, etc.)
#[derive(Debug)]
pub struct SearchParameters {
    // Total number of object params fully matched this seed (inc. COUNT)
    pub(crate) object_matches: usize,
    pub(crate) object_match_target: usize,
    // Total number of successfully-matched searches (seeds w/all params met)
    pub(crate) search_matches: u8,
    pub(crate) search_match_target: u8,
    pub(crate) debug: bool,
    pub(crate) depth_min: u8,
    pub(crate) depth_max: u8,
    pub(crate) file_paths: Vec<PathBuf>,
    pub(crate) format: FileFormat,
    pub(crate) seed_min:  u32,
    pub(crate) seed_max:  u32,
    pub(crate) verbosity: u8,
    pub(crate) object_params: Vec<ObjectParameter>,
}

impl SearchParameters {
    /// Creates a new instance from command line matches.
    pub(crate) fn from_matches(matches: clap::ArgMatches) -> Result<Self> {
        // Hold unwrapped search parameters
        let mut object_params = Vec::with_capacity(3);

        // --- General Values --- //    
        // MINDEPTH has default of 1, so always present.  Cannot be > MAXDEPTH
        let depth_min_val = matches.value_of("depth_min").unwrap();
        let depth_min = match depth_min_val.parse::<u8>() {
            Ok(val) => val,
            Err(_) => return Err(anyhow!("--mindepth must be from 1 to 26")),
        };

        // MAXDEPTH has default of 6, so always present.  Cannot be < MINDEPTH
        let depth_max_val = matches.value_of("depth_max").unwrap();
        let depth_max = match depth_max_val.parse::<u8>() {
            Ok(val) => val,
            Err(_) => return Err(anyhow!("--maxdepth must be from 1 to 26")),
        };

        if depth_min > depth_max { 
            return Err(anyhow!("--mindepth cannot be greater than --maxdepth"));
        }

        // MAXMATCHES has default of 10, so always present.  Must be 1 to 255.
        let max_matches_val = matches.value_of("matches_max").unwrap();
        let search_match_target = match max_matches_val.parse::<u8>() {
            Ok(val) => val,
            Err(_) => return Err(anyhow!("--matches must be from 1 to 255")),
        };

        // MINSEED has default of 1, so always present.  Cannot be > MAXSEED.
        let seed_min_val = matches.value_of("seed_min").unwrap();
        let seed_min = match seed_min_val.parse::<u32>() {
            Ok(val) => val,
            Err(_) => return Err(anyhow!("--minseed must be from 1 to 4294967295")),
        };        

        // MAXSEED has default of u32::MAX, so always present.  Cannot be < MINSEED.
        let seed_max_val = matches.value_of("seed_max").unwrap();
        let seed_max = match seed_max_val.parse::<u32>() {
            Ok(val) => val,
            Err(_) => return Err(anyhow!("--maxseed must be from 1 to 4294967295")),
        };        

        if seed_min > seed_max { 
            return Err(anyhow!("--minseed cannot be greater than --maxseed"));
        }

        // DEBUG defaults to `false`
        let debug = matches.is_present("debug");

        // FORMAT assumes UTF-16LE (default CE format) unless UTF-8 is specified.
        // If no files of the format are found, the formatting is switched 
        // (from `format_arg` to `format`).
        let format_arg = match matches.is_present("utf8") {
            true => FileFormat::Utf8,
            false => FileFormat::Utf16,        
        };

        // FILEPATH in which .csv files are found. Defaults to CWD if not given.  
        // Returned paths are UTF-16LE (Brogue CE format) unless UTF-8 is specified.
        let path = match matches.is_present("filepath") {
            true => Path::new(matches.value_of("filepath").unwrap()).into(),
            false => current_dir()?,
        };
        let (mut file_paths, format) = get_brogue_csv_paths(path, 0, format_arg)?;

        // RANDOM, if set, shuffles the list of file paths.
        if matches.is_present("random") {
            fastrand::shuffle(&mut file_paths);
        }

        // VERBOSITY can be from 1 to 3, and has default of 3 (always present).
        let verbosity: u8 = match matches.occurrences_of("verbose") {
            1 => 1,
            2 => 2,
            _ => 3,
        };

        // --- Ally --- //    
        if let Some(values) = matches.values_of("ally") {
            for search_result in parse_allies(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }

        // --- Altar --- //    
        if let Some(values) = matches.values_of("altar") {
            for search_result in parse_altars(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }
        
        // --- Armor --- //    
        if let Some(values) = matches.values_of("armor") {
            for search_result in parse_armors(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }

        // --- Charm --- //    
        if let Some(values) = matches.values_of("charm") {
            for search_result in parse_charms(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }

        // --- Food --- //    
        if let Some(values) = matches.values_of("food") {
            for search_result in parse_food(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }  
        
        // --- Gold --- //    
        if let Some(values) = matches.values_of("gold") {
            for search_result in parse_gold(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }        

        // --- Potion --- //    
        if let Some(values) = matches.values_of("potion") {
            for search_result in parse_potions(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }   

        // --- Ring --- //    
        if let Some(values) = matches.values_of("ring") {
            for search_result in parse_rings(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }     
        
        // --- Scroll --- //    
        if let Some(values) = matches.values_of("scroll") {
            for search_result in parse_scrolls(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }           

        // --- Staff --- //    
        if let Some(values) = matches.values_of("staff") {
            for search_result in parse_staves(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }   

        // --- Wand --- //    
        if let Some(values) = matches.values_of("wand") {
            for search_result in parse_wands(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }           

        // --- Weapon --- //
        if let Some(values) = matches.values_of("weapon") {
            for search_result in parse_weapons(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }
    
        // --- Equipment --- //
        if let Some(values) = matches.values_of("equipment") {
            for search_result in parse_equipment(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }

        // --- Items --- //
        if let Some(values) = matches.values_of("item") {
            for search_result in parse_items(values).into_iter() {
                match search_result {
                    Ok(param) => object_params.push(param),
                    Err(e) => return Err(e),
                }
            }
        }        

        // If any params are duplicates ("scale scale"), return an error
        let slice = &object_params;
        if (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1])) {
            return Err(anyhow!("Duplicate parameters detected (e.g. '-a scale scale'"));
        }

        Ok(
            Self {
                object_matches: 0,
                object_match_target: object_params.len(),  
                search_matches: 0,
                search_match_target,                  
                debug,
                depth_min,
                depth_max,
                file_paths,
                format,
                seed_min,
                seed_max,
                verbosity,
                object_params,
            }
        )
    }
    /// Clears `object_matches` field and `count` field of all ObjectParameters.
    pub fn clear(&mut self) {
        self.object_matches = 0;
        for obj_param in self.object_params.iter_mut() {
            obj_param.clear();
        }
    }
    /// Returns `true` if the search if the requested number of matches (set by
    /// '--matches' option has been met.
    pub(crate) fn is_complete(&self) -> bool {
        self.search_matches == self.search_match_target
    }          
    /// Returns `true` if all ObjectParameters are valid according to their `CountType`.
    /// A Search is valid if:
    /// - object_matches == object_match_target
    /// - EqualTo object parameters have count == count_target
    /// - LessThan object parameters have count < count_target
    pub(crate) fn is_valid(&self) -> bool {
        self.object_params.iter().all(|p| p.is_valid())
    }  
    /// Processes state of matches for the search and returns appropriate status.
    pub(crate) fn search_status(&mut self, match_resp: MatchResponse) -> SearchStatus {
        match match_resp {
            MatchResponse::Increment => {
                self.object_matches += 1;

                match self.object_matches == self.object_match_target {
                    false => SearchStatus::InProgress,
                    true => SearchStatus::AllObjectsFound,
                }
            }
            MatchResponse::EarlyExit => {
                SearchStatus::EarlySeedExit
            } 
            MatchResponse::DoNothing => SearchStatus::InProgress,
        }
    }    
    /// Manually sets file to open.  Used for testing.
    #[allow(dead_code)]
    pub(crate) fn set_file(&mut self, file: &str) {
        let file = PathBuf::from(file);
        self.file_paths.clear();
        self.file_paths.push(file);
    }                                      
}

impl Default for SearchParameters {
    fn default() -> Self {
        Self {
            object_matches: 0,
            object_match_target: 0,   
            search_matches: 0,
            search_match_target: 10,   
            debug: false,              
            depth_min: 1,
            depth_max: 6,
            file_paths: Vec::new(),
            format: FileFormat::Utf8,
            seed_min: 1,
            seed_max: u32::MAX,
            verbosity: 3,
            object_params: Vec::new(),
        }
    }
}

impl std::fmt::Display for SearchParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Search:\n")?;

        write!(f, " verbosity: {}\n", self.verbosity)?;

        match self.format {
            FileFormat::Utf8 => write!(f, "    format: UTF-8\n")?,
            FileFormat::Utf16 => write!(f, "    format: UTF-16LE\n")?,
        }

        write!(f, "     depth: {} to {}\n", self.depth_min, self.depth_max)?;
        write!(f, "      seed: {} to {}\n", self.seed_min, self.seed_max)?;
        write!(f, "Objects:\n")?;
        
        for param in self.object_params.iter() {
            write!(f, "{}", param)?;
        }

        Ok(())
    }
}

/// Checks if `PrepParam` struct is valid `SearchParameter` based on `Category`.
/// If so, converts it and adds to Vec of parameters. Most categories need only be 
// non-empty (at least one value is `Some` or `true`).
///
/// `Food` and `Gold` require `COUNT` to be present and returns `Err` if missing.
pub fn add_parameter(
    category: Category,
    prep: &mut PrepParams, 
    params: &mut Vec<Result<ObjectParameter>>,
) {
    use Category::*;

    match category {
        Food | Gold => {
            if prep.count.is_none() {
                params.push(
                    Err(anyhow!("COUNT is required for the '{}' category", category))
                );    
            }
        }
        _ => {
            if prep.is_empty() {
                params.push(
                    Err(anyhow!("Insufficient/invalid parameters for '{}' category", category))
                ); 
            }
        },
    }

    let param = Ok(ObjectParameter::from_prep(category, prep));
    params.push(param);
    *prep = PrepParams::new();
}
