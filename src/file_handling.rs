use anyhow::{anyhow, Result};
use encoding_rs::Encoding;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// The two file formats that can be used for Brogue CSVs.  Files produced by the
/// Brogue CE executable produce files in UTF-16LE format, while Rust takes UTF-8 for
/// its strings (used by CSV readers).
#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    Utf8,
    Utf16, 
}

impl FileFormat {
    // Returns other format (Utf8 -> Utf16; Utf16 -> Utf8).
    fn toggled(&self) -> Self {
        match self {
            FileFormat::Utf8 => FileFormat::Utf16,
            FileFormat::Utf16 => FileFormat::Utf8,
        }
    }
}

/// Gets list of valid Brogue seed CSV files for a given folder path.  Attempts to
/// gather files of the specified format (default UTF-16LE), but if no files found,
/// will switch to the other format (UTF-8).
///
/// Also returns the format that was ultimately chosen (in case intended one failed).
pub fn get_brogue_csv_paths<P>(
    path: P, 
    nesting_max: usize, 
    format: FileFormat,
) -> Result<(Vec<PathBuf>, FileFormat)>  
where 
    P: AsRef<Path> + Clone + Debug
{
    let paths = get_csv_paths(path.clone(), nesting_max, format)?;

    match paths.is_empty() {
        false => Ok((paths, format)),
        true => {
            let paths = get_csv_paths(path.clone(), nesting_max, format.toggled())?;
            Ok((paths, format.toggled()))
        }
    } 
}

/// Gets list of valid Brogue seed CSV files for a given folder path. Can search 
/// in nested folders.
fn get_csv_paths<P>(
    path: P, 
    nesting_max: usize, 
    format: FileFormat,
) -> Result<Vec<PathBuf>>  
where 
    P: AsRef<Path> + Debug
{
    let mut nesting_lvl: usize = 0;
    let file_exts = ["csv"];
    let mut result: Vec<PathBuf> = Vec::new();

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {

                let path = entry.path();              
                if path.is_dir() {
                    if nesting_lvl < nesting_max {
                        nesting_lvl += 1;
                        if let Ok(nested) = get_csv_paths(&path, nesting_max, format) {
                            result.extend(nested.iter().cloned()); 
                        }
                    }
                } else {
                    // Find all files with matching extensions
                    if path.extension().is_none() {
                        continue;
                    }
                    let ext = path.extension().unwrap().to_str().expect("UTF-8");
                    if file_exts.contains(&ext) {
                        if is_valid_csv_format(&path, format) {
                            result.push(path);
                        }            
                    }
                }
            }
        }
    } else {
        return Err(anyhow!("couldn't find files in path {:?}", &path));
    }
    Ok(result)
}

/// Validates a proper Brogue seed catalog file by checking file format.
///
/// CSV file is valid if:
/// - it loads w/o error (File::open().is_ok())
/// - File format matches specified format (UTF-8 / UTF-16LE by Byte Order Mark (BOM))
///
/// Note that this is a non-exhaustive, perfunctory check.  Headers are checked in the 
/// `search_files()` function.
fn is_valid_csv_format<P>(path: P, format: FileFormat) -> bool 
where 
    P: AsRef<Path> + Debug
{    
    if let Ok(f) = File::open(&path) {
        let mut reader = BufReader::with_capacity(10, f);
        reader.fill_buf().unwrap();    
        let buffer = reader.buffer();

        return match (format, Encoding::for_bom(buffer)) {
            (FileFormat::Utf16, Some(encoding)) => encoding.0 == encoding_rs::UTF_16LE,
            (FileFormat::Utf16, None) => false,
            (FileFormat::Utf8, Some(_)) => true,
            (FileFormat::Utf8, None) => true,
        }
    }

    false
}
