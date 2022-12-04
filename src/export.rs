use crate::exiv2::ScriptGenerator;
use crate::hash::hash_file;
use crate::iphoto::{AlbumData, Image};
use crate::progress;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

const CHARS_TO_FIX: [char; 2] = ['*', '/'];

pub(crate) fn export_albums(message: &str, album_data: AlbumData, stage_path: &PathBuf) {
    info!("{} albums to stage", album_data.albums.len());

    let mut script_generator = ScriptGenerator::new(stage_path);

    let (tx, rx) = mpsc::channel();

    let total_photos_to_copy = album_data
        .albums
        .iter()
        .fold(0, |acc, album| acc + album.key_list.len());

    let progress_handle = progress::create_progress_thread(total_photos_to_copy, message, rx);

    album_data.albums.iter().for_each(|album| {
        let mut album_path = PathBuf::from(stage_path);

        if album.name.contains(CHARS_TO_FIX) {
            info!("Fixing album name: {}", album.name);
            album_path.push(album.name.replace(CHARS_TO_FIX, "-"));
        } else {
            album_path.push(&album.name);
        }

        info!(r#"Creating album "{}""#, album.name);

        debug!(
            "Creating directory {}",
            album_path.to_str().expect("converting path to str")
        );
        fs::create_dir(&album_path).expect("creating directory");

        copy_photos(
            &album_path,
            &album.key_list,
            &album_data.master_image_map,
            &tx,
            &mut script_generator,
        );
    });

    progress_handle.join().expect("joining thread");
}

fn copy_photos(
    album_path: &Path,
    photo_ids: &[String],
    master_image_map: &HashMap<String, Image>,
    tx: &Sender<usize>,
    script_generator: &mut ScriptGenerator,
) {
    photo_ids.iter().for_each(|id| {
        tx.send(1).expect("sending progress");

        if let Some(image) = master_image_map.get(id) {
            let image_path = Path::new(&image.image_path);

            if image_path.exists() {
                let mut album_path = PathBuf::from(album_path);
                album_path.push(format!(
                    "{}_{}",
                    id,
                    image_path
                        .file_name()
                        .expect("getting file name")
                        .to_str()
                        .unwrap()
                ));

                debug!(
                    "Copying {} to {}",
                    &image.image_path,
                    &album_path.to_str().expect("converting path to str")
                );
                fs::copy(image_path, &album_path).expect("copying photo");

                match hash_file(image_path) {
                    Ok(copy_hash) => {
                        if let Some(image_hash) = image.hash {
                            if copy_hash != image_hash {
                                error!("Checksum mismatch for {}", &image.image_path);
                            }
                        } else {
                            error!("Copied a file that did not have a checksum");
                        }
                    }
                    Err(error) => error!(
                        "There was an error generating a hash for {}: {}",
                        image_path.as_os_str().to_str().unwrap(),
                        error
                    ),
                }

                if image.media_type == "Image" && !image.comment.is_empty() {
                    script_generator.add_comment(&image.comment, album_path.to_str().unwrap());
                }
            } else {
                warn!("{} does not exist", &image.image_path);
            }
        } else {
            error!("Could not find image for {id}");
        }
    });
}
