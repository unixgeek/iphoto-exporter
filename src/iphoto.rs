use plist::Dictionary;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use log::{debug, info};

const COMPONENTS: [&str; 2] = ["Modified", "Originals"];

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AlbumDataInternal {
    archive_id: String,
    #[serde(rename = "List of Albums")]
    list_of_albums: Vec<Album>,
    #[serde(rename = "Master Image List")]
    master_image_list: Dictionary,
}

pub(crate) struct AlbumData {
    pub(crate) archive_id: String,
    pub(crate) albums: Vec<Album>,
    pub(crate) master_image_map: HashMap<String, Image>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Album {
    #[serde(rename = "AlbumName")]
    pub(crate) name: String,
    #[serde(rename = "Album Type")]
    pub(crate) r#type: Option<String>,
    pub(crate) key_list: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct Image {
    pub(crate) media_type: String,
    pub(crate) comment: String,
    pub(crate) image_path: String,
    pub(crate) original_path: Option<String>,
    pub(crate) hash: Option<u32>,
}

pub(crate) fn parse_album_data(album_data_xml_path: &PathBuf) -> AlbumData {
    let album_data: AlbumDataInternal = plist::from_reader(BufReader::new(
        File::open(album_data_xml_path).expect("opening plist file"),
    ))
    .expect("parsing plist file");

    let mut album_data = from_album_data_internal(album_data);

    fix_paths(&mut album_data, album_data_xml_path);

    log_details(&album_data);

    album_data
}

fn log_details(album_data: &AlbumData) {
    info!("Total albums: {}", album_data.albums.len());

    debug!("Archive Id: {}", album_data.archive_id);

    debug!("== Albums ==");
    album_data.albums.iter().for_each(|album| {
        debug!("{album:#?}");
    });

    debug!("== Images ==");
    album_data.master_image_map.iter().for_each(|(id, image)| {
        debug!("id: {id}, {image:#?}");
    });
}

fn fix_paths(album_data: &mut AlbumData, album_data_xml_path: &PathBuf) {
    let base_dir = Path::new(album_data_xml_path)
        .parent()
        .expect("getting base directory")
        .canonicalize()
        .expect("canonicalizing path");

    album_data
        .master_image_map
        .iter_mut()
        .for_each(|(_, image)| {
            image.image_path = fix_path(&base_dir, &image.image_path);

            if let Some(original_path) = &image.original_path {
                image.original_path = Some(fix_path(&base_dir, original_path));
            }
        });
}

fn from_album_data_internal(album_data_internal: AlbumDataInternal) -> AlbumData {
    let master_image_map = album_data_internal
        .master_image_list
        .into_iter()
        .map(|(key, image)| {
            // Why can't I use serde on the value?
            let mut dict = image.into_dictionary().expect("converting to a dictionary");

            let media_type = dict
                .remove("MediaType")
                .expect("getting MediaType value")
                .into_string()
                .unwrap();
            let comment = dict
                .remove("Comment")
                .expect("getting Comment value")
                .into_string()
                .unwrap();
            let image_path = dict
                .remove("ImagePath")
                .expect("getting ImagePath value")
                .into_string()
                .unwrap();
            let original_path = dict
                .remove("OriginalPath")
                .map(|value| value.into_string().unwrap());
            let image = Image {
                media_type,
                comment,
                image_path,
                original_path,
                hash: None,
            };

            (key, image)
        })
        .collect();

    AlbumData {
        archive_id: album_data_internal.archive_id,
        albums: album_data_internal.list_of_albums,
        master_image_map,
    }
}

fn fix_path(base_path: &Path, original_path: &str) -> String {
    let original_path = Path::new(original_path);
    let mut real_path = PathBuf::from(base_path);
    let mut found = false;
    for c in original_path.components() {
        if found || COMPONENTS.contains(&c.as_os_str().to_str().unwrap()) {
            found = true;
            real_path.push(c);
        }
    }

    real_path.to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::iphoto::fix_path;
    use std::path::Path;

    #[test]
    fn test_fix_path() {
        assert_eq!("/tmp/Originals/2005/2005-10-05 Hannah at Play/DCP_1934.jpg".to_string(), fix_path(Path::new("/tmp"),"/Users/darlene/Pictures/iPhoto Library/Originals/2005/2005-10-05 Hannah at Play/DCP_1934.jpg"));
    }
}
