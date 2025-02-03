use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_bytes<P>(filename: P, num_bytes: usize) -> Result<String, String>
where
    P: AsRef<Path>,
{
    let mut file: File = File::open(&filename).map_err(|e| {
        format!(
            "Failed to open file {} : {}",
            filename.as_ref().display(),
            e
        )
    })?;

    let mut buffer = Vec::with_capacity(num_bytes);
    buffer.resize(num_bytes, 0);
    file.read_exact(&mut buffer)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Extract String from bytes vector
    let mut content = String::from_utf8(buffer).map_err(|e| format!("Invalid UTF-8 sequence: {}", e))?;

    // Remove BOM if present
    if content.starts_with('\u{feff}') {
        content = content.replacen('\u{feff}', "", 1);
    }

    Ok(content)
}
