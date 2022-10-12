use std::collections::HashSet;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;

pub fn find_resp_file_by_uri(
    uri: &str,
    allowed_extensions: &HashSet<String>,
) -> Result<(String, String), Box<dyn Error>> {
    let fuck_path = uri.replacen('/', "", 1);
    let sanitized_path = fuck_path
        .split('/')
        .filter(|&x| x != "." && x != "..")
        .collect::<Vec<_>>();
    let file_extension = Path::new(sanitized_path.last().unwrap())
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    if !allowed_extensions.contains(file_extension) {
        return Err(format!("file extension '{}' disallowed", file_extension).into())
    }
    let concatenated_path = sanitized_path.join("/");
    let file_content = read_to_string(&concatenated_path)?;
    Ok((concatenated_path, file_content))
}
