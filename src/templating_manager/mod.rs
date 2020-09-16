use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::{from_utf8, FromStr};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParsingError {
    /// Wrapper for std::io::Error which can be thrown when reading in files
    IOError(std::io::Error),
    /// Wrapper for std::str::Utf8Error which can be thrown if files are not UTF8 encoded
    UTF8EncodingError(std::str::Utf8Error),
    /// A path in a template could not be resolved to a valid path
    MalformedTemplatePath(String),
    // @TODO Check for this error by looking for cycles in a tree
    RecursivePath(PathBuf),
}

impl Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            ParsingError::IOError(error) => std::fmt::Display::fmt(&error, f),
            ParsingError::UTF8EncodingError(error) => std::fmt::Display::fmt(&error, f),
            ParsingError::MalformedTemplatePath(path) => {
                write!(f, "Malformed template path encountered: {}", path)
            }
            ParsingError::RecursivePath(path) => {
                write!(f, "Recursive template path call from {}", path.to_str().unwrap())
            }
        }
    }
}

impl From<std::io::Error> for ParsingError {
    fn from(error: std::io::Error) -> Self {
        ParsingError::IOError(error)
    }
}

impl From<std::str::Utf8Error> for ParsingError {
    fn from(error: std::str::Utf8Error) -> Self {
        ParsingError::UTF8EncodingError(error)
    }
}

/// Parse a file for templates and return its processed contents as a vector of bytes.
///
/// Templates are expected in the following format: {{../relative/path/to/file.extension}}
///
/// # Arguments
///
/// * `filepath` - Path of the file to begin parsing.
///
pub fn parse(filepath: &Path) -> Result<Vec<u8>, ParsingError> {

    // set up hashmap with owned (canonicalized) paths of parsed files
    let mut parsed_files: HashMap<PathBuf, Vec<u8>> = HashMap::new();

    // call recursive function
    parse_recursive(&mut parsed_files, filepath)
}

// smart recursive function used for parsing files
fn parse_recursive(parsed_files: &mut HashMap<PathBuf, Vec<u8>>, filepath: &Path) -> Result<Vec<u8>, ParsingError> {

    // buffer storing parsed file contents
    let mut result: Vec<u8> = Vec::new();

    let absolute_path = filepath.canonicalize()?;

    // check if we have already parsed this file
    if parsed_files.contains_key(absolute_path.as_path()) {
        result.append(&mut parsed_files[&absolute_path].clone());
        Ok(result)
    } else {

        // open and recursively parse the file
        let file = File::open(filepath)?;
        let mut reader = BufReader::new(file);
        while let Some(path) = find_template(&mut reader, &mut result)? {
            result.append(&mut parse_recursive(parsed_files, filepath.join(path).as_path())?);
        }

        // store the parsed file in the dictionary and return it
        parsed_files.insert(absolute_path.clone(), result.clone());
        Ok(result)
    }
}

// finds the first template (if any) in the bufreader stream and returns its path
// reads the file up to that template into the buffer
fn find_template(
    reader: &mut BufReader<File>,
    result: &mut Vec<u8>,
) -> Result<Option<PathBuf>, ParsingError> {

    // find template start {{
    if find_repeated_byte(reader, b'{', result)? {
        let open = result.len();

        // find template end }}
        if find_repeated_byte(reader, b'}', result)? {

            // get path from template
            let path_slice = result.split_off(open - 2);
            let path = from_utf8(&path_slice[2..path_slice.len() - 2])?;

            // convert to path
            return match PathBuf::from_str(path) {
                Ok(path_buf) => Ok(Some(path_buf)),
                Err(_) => Err(ParsingError::MalformedTemplatePath(path.to_string())),
            };
        }
    }

    Ok(None)
}

// checks for the first instance of the given byte (if any) in the bufreader stream
// reads the file until this instance into the buffer
fn find_repeated_byte(
    reader: &mut BufReader<File>,
    byte: u8,
    result: &mut Vec<u8>,
) -> Result<bool, ParsingError> {

    // read until the first byte is found (if any)
    let num_bytes_first = reader.read_until(byte, result)?;
    if num_bytes_first == 0 {
        return Ok(false);
    }

    // loop until the second byte is found and check if it is adjacent to first
    loop {
        let num_bytes_second = reader.read_until(byte, result)?;

        // byte never found
        if num_bytes_second == 0 {
            return Ok(false);

        // adjacent byte found
        } else if num_bytes_second == 1 {
            return Ok(true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir, remove_dir_all, write};

    #[test]
    fn test_parse_multiple_templates() -> Result<(), Box<dyn Error>> {
        let test_dir = Path::new("./test_templates_1");
        create_dir(test_dir)?;

        let test_template = test_dir.join(Path::new("test_template.txt"));
        write(
            test_template.clone(),
            "Some template { sike } i guess this is a template this time {{../test_template2.txt}} and {{../test_template3.txt}} then } } }} this { {{ stuff over here",
        )?;

        let test_template_two = test_dir.join(Path::new("test_template2.txt"));
        write(test_template_two, "some random text {{../test_template3.txt}}")?;

        let test_template_three = test_dir.join(Path::new("test_template3.txt"));
        write(test_template_three, "or other shit")?;

        let res = parse(test_template.as_path())?;
        assert_eq!("Some template { sike } i guess this is a template this time some random text or other shit and or other shit then } } }} this { {{ stuff over here", std::str::from_utf8(&*res).unwrap());
        remove_dir_all(test_dir)?;
        Ok(())
    }

    #[test]
    fn test_parse_no_templates() -> Result<(), Box<dyn Error>> {
        let test_dir = Path::new("./test_templates_2");
        create_dir(test_dir)?;

        let test_template = test_dir.join(Path::new("test_template.txt"));
        write(
            test_template.clone(),
            "Some template { sike } i guess then } } }} this { {{ stuff over here",
        )?;

        let test_template_two = test_dir.join(Path::new("test_template2.txt"));
        write(test_template_two.clone(), "")?;

        let res_no_template = parse(test_template.as_path())?;
        assert_eq!(b"Some template { sike } i guess then } } }} this { {{ stuff over here".to_vec(), res_no_template);

        let res_empty = parse(test_template_two.as_path())?;
        assert_eq!(b"".to_vec(), res_empty);

        remove_dir_all(test_dir)?;
        Ok(())
    }
}
