use crate::iphoto::Image;
use crate::progress;
use crc32fast::Hasher;
use log::{error, warn};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::mpsc;

pub(crate) fn generate_image_hash(message: &str, master_image_map: &mut HashMap<String, Image>) {
    let (tx, rx) = mpsc::channel();

    let progress_handle = progress::create_progress_thread(master_image_map.len(), message, rx);

    master_image_map.values_mut().for_each(|image| {
        tx.send(1).expect("sending progress");

        let image_path = Path::new(&image.image_path);

        if !image_path.exists() {
            warn!(
                "{} does not exist",
                image_path.as_os_str().to_str().unwrap()
            );
        } else {
            match hash_file(image_path) {
                Ok(value) => image.hash = Some(value),
                Err(error) => error!(
                    "There was an error generating a hash for {}: {}",
                    image_path.as_os_str().to_str().unwrap(),
                    error
                ),
            }
        }
    });

    progress_handle.join().expect("joining thread");
}

pub(crate) fn hash_file(file: &Path) -> Result<u32, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(file)?);
    let mut hasher = Hasher::new();
    let mut buffer: [u8; 8192] = [0; 8192];
    let mut result;
    loop {
        result = reader.read(&mut buffer)?;
        if result == 0 {
            break;
        }
        hasher.update(&buffer[..result]);
    }

    Ok(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use crate::hash::hash_file;
    use std::path::Path;

    #[test]
    fn test_hash_file() {
        assert_eq!(2947800163, hash_file(Path::new("test.txt")).unwrap());
    }
}
