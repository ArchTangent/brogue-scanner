//! Testing for Brogue CE Scanner.

// TODO: combined search for each Object type (e.g. -a, -a, -a)
// TODO: search with different Object categories (e.g. -a, -w, -p)

use crate::*;

const FILE: &str = "./src/test_data.csv";

// Checks number of *lines* (not seeds) that match the armor query.
// --armor [COUNT] [ENCHANTMENT] [DEPTH] [KIND] [MAGIC] [RUNIC] [VAULT] {"runic"}
#[test]
fn armor() {
    let args = &[
        "brogue-scanner", 
        "-a", "scale"
    ];
    let matches = new_app().get_matches_from(args);
    let mut search = SearchParameters::from_matches(matches).unwrap();
    search.set_file(FILE);

    let search_matches = search_files(&mut search).unwrap();
    let match_count = search_matches.len();

    assert_eq!(match_count, 7);
}