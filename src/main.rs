//! Brogue Seed Scanner.
//!
//! Takes .csv data generated from Brogue (Community Edition), and uses it to find user-selected 
//! search criteria, such as seeds with a +3 or higher weapon of paralysis and >5 enchantment
//! scrolls within the first 6 dungeon levels.
//!
//! Usage (Windows & Powershell):
//! 
//! Create a Brogue seed .csv file. From the Brogue CE game folder:
//! ```
//! brogue-cmd --csv --print-seed-catalog 2001 1000 26 > 2001-3000_utf16.csv
//! ```
//! The above will export data for:
//! - 1000 seeds
//! - starting at seed 2001
//! - checking the first 26 dungeon floors
//! - and exporting the data (via redirection operator ">") to a file called "2001-3000_utf16.csv".
//!
//! The resulting file will be in UTF-16 format.
//! 
//! Optionally, you can halve the size of each CSV by coverting them to UTF-8:
//! ```
//! Get-Content .\2001-3000_utf16.csv | Set-Content -Encoding utf8 .\2001-3000.csv
//! ```
//! 
//! Finally, search using `brogue-scanner`, run from the same folder as your CSV:
//! ```
//! brogue-scanner "-a scale +2 mutuality"
//! ```
//! 
//! The above query searches for: 1 or more that `scale mail` armor items with the 'mutuality' runic 
//! and `+2` enchantment level.

mod bitflags;
mod file_handling;
mod objects;
mod search;
#[cfg(test)]
mod tests;

use anyhow::Result;
use clap::{App, Arg};
use search::{SearchParameters, search_files, display_matches};

/// Creates a new instance of a `brogue-scanner` app.
pub(crate) fn new_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Brogue Seed Scanner")
        .version("0.9.6")
        .author("ArchTangent")
        .about("Search Brogue CE seeds for items and allies")     
        // --- GENERAL --- //
        .arg(Arg::with_name("debug")
            .short("D")
            .long("debug")
            .help("If set, debug information will be printed during the search.")
        )
        .arg(Arg::with_name("depth_min")
            .long("mindepth")
            .value_name("DEPTH")
            .default_value("1")
            .help("Minimum dungeon depth to search from 1 to 26.")
        )
        .arg(Arg::with_name("depth_max")
            .short("d")        
            .long("depth")
            .alias("maxdepth")
            .value_name("DEPTH")
            .default_value("26")
            .help("Maximum dungeon depth to search, from 1 to 26.")
        )
        .arg(Arg::with_name("filepath")
            .short("F")
            .long("--filepath")
            .value_name("FILEPATH")
            .help("Filepath in which seed catalog .csv files are found. Defaults\n\
                  to the current working directory.")
        )        
        .arg(Arg::with_name("matches_max")
            .short("m")        
            .long("matches")
            .value_name("MATCHES")
            .default_value("10")
            .help("Maximum number of matching seeds to return, from 1 to 255.")
        )
        .arg(Arg::with_name("random")
            .short("R")
            .long("random")
            .help("If set, csv files will be checked in random order.")
        )        
        .arg(Arg::with_name("seed_min")
            .long("minseed")
            .alias("start")
            .value_name("SEED")
            .default_value("1")
            .help(
                "Minimum dungeon seed to search, from 1 to 4294967295.  \
                Cannot exceed --maxdepth."
            )
        )
        .arg(Arg::with_name("seed_max")
            .long("maxseed")
            .alias("stop")
            .value_name("SEED")
            .default_value("4294967295")
            .help(
                "Maximum dungeon seed to search, from 1 to 4294967295.  \
                Cannot be less than --minseed."
            )
        )
        .arg(Arg::with_name("utf8")
            .short("U")
            .long("utf8")
            .conflicts_with("utf16")
            .help(
                "When set, searches for CSV files in UTF-8 format (normally UTF-16).  \
                Seed catalogs produced by Brogue CE are in UTF-16 format."
            )
        )
        .arg(Arg::with_name("utf16")
            .long("utf16")
            .conflicts_with("utf8")
            .help(
                "When set, searches for CSV files in UTF-16 format (the default).  \
                Seed catalogs produced by Brogue CE are in UTF-16 format."
            )
        )
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help(
                "Sets search verbosity from 1 to 3 (-v, -vv or -vvv), default '3'.\n  \
                  Level 3: display seeds + depths + matches\n  \
                  Level 2: display seeds + depths\n  \
                  Level 1: display seeds"
            )
        )   
        // --- CATEGORIES --- //    
        .arg(Arg::with_name("ally")
            .short("A")
            .long("ally")
            .value_name("ALLY")
            .min_values(1)
            .multiple(true)
            .help(
                "Allies matching [COUNT] [DEPTH] [KIND] [MUTATION] [STATUS] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  KIND: any monster kind ('dar', 'troll').  Partial match allowed.\n  \
                  MUTATION: any valid mutation (e.g. 'toxic').  Partial match allowed.\n  \
                  STATUS: 'shackled', 'caged', or 'legendary'.\n\
                Special Term(s):\n  \
                  'mutation': finds allies with any mutation\n\
                Examples:\n  \
                  '--ally explosive goblin'\n  \
                  '--ally 2 legendary'"
            )
        )                           
        .arg(Arg::with_name("altar")
            .long("altar")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Altars matching [COUNT] [DEPTH] [KIND], in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  KIND: 'commutation' or 'resurrection'. Partial match allowed.\n  \
                Examples: \n  \
                  '--altar 2 comm'\n  \
                  '--altar resurrection'"
            )
        )        
        .arg(Arg::with_name("armor")
            .short("a")
            .long("armor")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Armor matching [COUNT] [DEPTH] [ENCHANTMENT] [KIND] [MAGIC] [RUNIC] [VAULT] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  ENCHANTMENT: integer in form +N or N- ('+3', '+0', '-1'). Default 'any.'\n    \
                    (+N) : find objects with enchantment >= N\n    \
                    (N-) : find objects with enchantment <= N\n\
                  KIND: any armor kind (e.g. 'scale'). Partial match allowed.\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  RUNIC: any armor runic (e.g. 'goblin'). Partial match allowed.\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Special Term(s):\n  \
                  'runic': finds any runic armor matching specified params.\n\
                Examples: \n  \
                  '--armor 2 +3 scale mutuality'\n  \
                  '--armor 1- chain immolation'\n  \
                  '--armor +2 runic'"
            )
        )
        .arg(Arg::with_name("charm")
            .short("c")
            .long("charm")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Charms matching [COUNT] [DEPTH] [ENCHANTMENT] [KIND] [VAULT] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  ENCHANTMENT: integer in form +N ('+3', '+0'). Default 'any'.\n    \
                    (+N) : find objects with enchantment >= N\n  \
                  KIND: any charm kind (e.g. 'protection'). Partial match allowed.\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Examples: \n  \
                  '--charm 1 +3 invisibility'\n  \
                  '--charm telepathy'"
            )
        )
        .arg(Arg::with_name("equipment")
            .short("e")
            .long("equipment")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Equipment matching [COUNT] [DEPTH] [ENCHANTMENT] [MAGIC] [VAULT] in any order. \
                Equipment includes object you can equip (armor, rings, and weapons).\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  ENCHANTMENT: integer in form +N or N- ('+3', '+0', '1-'). Default 'any.'\n    \
                    (+N) : find objects with enchantment >= N\n    \
                    (N-) : find objects with enchantment <= N\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Examples: \n  \
                  '--equipment 2 +3'\n  \
                  '--equipment good vault'\n  \
                  '--equipment runic'"
            )
        )        
        .arg(Arg::with_name("food")
            .short("f")
            .long("food")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Food matching <COUNT> [DEPTH] [KIND] in any order.\n\
                  COUNT: quantity (e.g. '2'). Required. Default '1'. Max 255.\n\
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  KIND: 'mango' or 'food'. Partial match allowed.\n\
                Examples: \n\
                  '--food 5 mango'\n\
                  '--food 12'"
            )
        )
        .arg(Arg::with_name("gold")
            .short("g")
            .long("gold")
            .value_name("COUNT")
            .help(
                "Find seeds with at least <COUNT> amount of gold.\n\
                Example: \n\
                  '--gold 2600'"
            )
        )
        .arg(Arg::with_name("item")
            .short("i")
            .long("item")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Items matching [COUNT] [DEPTH] [ENCHANTMENT] [MAGIC] [VAULT] in any order. \
                Items are any object that can be found in a vault:  armor, charms, potions, \
                rings, scrolls, wands, and weapons.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  ENCHANTMENT: integer in form +N or N- ('+3', '+0', '1-'). Default 'any.'\n    \
                    (+N) : find objects with enchantment >= N\n    \
                    (N-) : find objects with enchantment <= N\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Examples: \n  \
                  '--item 2 +3'\n  \
                  '--item good vault'\n  \
                  '--item runic'"
            )
        )                   
        .arg(Arg::with_name("potion")
            .short("p")
            .long("potion")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Potions matching [COUNT] [DEPTH] [KIND] [MAGIC] [VAULT] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  KIND: any potion kind (e.g. 'life'). Partial match allowed.\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Examples: \n  \
                  '--potion 15'\n  \
                  '--potion 5 descent'"
            )
        )  
        .arg(Arg::with_name("ring")
            .short("r")
            .long("ring")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Rings matching [COUNT] [DEPTH] [ENCHANTMENT] [KIND] [MAGIC] [VAULT] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  ENCHANTMENT: integer in form +N or N- ('+3', '+0', '1-'). Default 'any.'\n    \
                    (+N) : find objects with enchantment >= N\n    \
                    (N-) : find objects with enchantment <= N\n  \
                  KIND: any ring kind (e.g. 'stealth'). Partial match allowed.\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Examples: \n  \
                  '--ring 1 +3 light'\n  \
                  '--ring 2- regeneration'\n  \
                  '--ring stealth'"
            )
        )    
        .arg(Arg::with_name("scroll")
            .short("S")
            .long("scroll")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Scrolls matching [COUNT] [DEPTH] [KIND] [MAGIC] [VAULT] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  KIND: any scroll kind (e.g. 'identify'). Partial match allowed.\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Examples: \n  \
                  '--scroll 8'\n  \
                  '--scroll 18 enchantment'"
            )
        )     
        .arg(Arg::with_name("staff")
            .short("s")
            .long("staff")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Staves matching [COUNT] [DEPTH] [ENCHANTMENT] [KIND] [MAGIC] [VAULT] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  ENCHANTMENT: integer in form +N ('+3', '+0'). Default 'any.'\n    \
                    (+N) : find objects with enchantment >= N\n  \
                  KIND: any staff kind (e.g. 'firebolt'). Partial match allowed.\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Examples: \n  \
                  '--staff 3 +2 lightning'\n  \
                  '--staff entrancement'"
            )
        )   
        .arg(Arg::with_name("wand")
            .short("W")
            .long("wand")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Wands matching [COUNT] [DEPTH] [ENCHANTMENT] [KIND] [MAGIC] [VAULT] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  ENCHANTMENT: integer in form +N ('+3', '+0'). Default 'any.'\n    \
                    (+N) : find objects with enchantment >= N. In the case of wands, this is the number of charges.\n  \
                  KIND: any wand kind (e.g. 'domination'). Partial match allowed.\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Examples: \n  \
                  '--wand 1 +2 plenty'\n  \
                  '--wand empowerment'"
            )
        )                      
        .arg(Arg::with_name("weapon")
            .short("w")
            .long("weapon")
            .value_name("PARAMS")
            .min_values(1)
            .multiple(true)
            .help(
                "Weapons matching [COUNT] [DEPTH] [ENCHANTMENT] [KIND] [MAGIC] [RUNIC] [VAULT] in any order.\n  \
                  COUNT: quantity (e.g. '2'). Default '1'. Max 255.\n  \
                  DEPTH: maximum dungeon depth to search for this object.\n  \
                  ENCHANTMENT: integer in form +N or N- ('+3', '+0', '1-'). Default 'any.'\n    \
                    (+N) : find objects with enchantment >= N\n    \
                    (N-) : find objects with enchantment <= N\n\
                  KIND: any weapon kind (e.g. 'spear'). Partial match allowed.\n  \
                  RUNIC: any weapon runic (e.g. 'paralysis'). Partial match allowed.\n  \
                  MAGIC: 'bad', 'good' - whether object is blessed or malevolent (default either).\n  \
                  VAULT: 'vault' or 'novault' - whether object is in vault (default either).\n\
                Special Term(s):\n  \
                  'runic': finds any runic weapon matching specified params.\n\
                Examples:\n  \
                  '--weapon 2 +3 whip quietus'\n  \
                  '--weapon sword mercy 1-'\n  \
                  '--weapon +2 runic'"
              )
          )
}

//  ##    ##     ##     ########  ##    ##
//  ###  ###   ##  ##      ##     ####  ##
//  ## ## ##  ##    ##     ##     ## ## ##
//  ##    ##  ########     ##     ##  ####
//  ##    ##  ##    ##  ########  ##    ##

//* To call find .csvs in ".\\src" folder, use "-F '.\\src'"
fn main() -> Result<()> {
    println!("\n=====  BROGUE SEED SCANNER  =====\n");
 
    let matches = new_app().get_matches();
        
    // --- Get Params and Perform Search --- //
    let mut search = SearchParameters::from_matches(matches)?;
    let search_matches = search_files(&mut search)?;

    display_matches(&search_matches, &search);

    Ok(())
}
