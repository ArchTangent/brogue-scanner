# To-Dos for Brogue CE Scanner

## To-Do List

1. Compression of files for smaller size:

    - compressing csv files with `gzip` reduces file size by ***~95%***
    - use `.BROGUECATALOG` or `.BROGUESEED` files that holds zipped csv files.
    - make a `--compress` command that will convert files to `.BROGUESEED`.
    - automatically make program recognize compressed files.

2. Update `clap` to version `4.0`

    - use `crate_version()!` macro for the crate version

3. Add functionality or remove functionality for `key` types.

4. Finalize versioning system

    - integer revisions: `v236`, `v237`, ...
    - semver: `1.0.1`, `2.1.3`, ...

## Alternatives

1. Database to store seed data
    - Convert seed data to database format
    - Use SQL queries to retrieve data.

## CSV Headers

0  dungeon_version
1  seed
2  depth
3  quantity
4  category
5  kind
6  enchantment
7  runic
8  vault_number
9  opens_vault_number
10 carried_by_monster_name
11 ally_status_name
12 mutation_name
