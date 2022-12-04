use crate::iphoto::AlbumData;
use log::info;
use std::collections::HashMap;
use std::fmt::Write as _;

pub(crate) fn log_photo_counts(album_data: &AlbumData) {
    let master_image_map_count = album_data.master_image_map.len();

    let master_album_image_count = album_data
        .albums
        .iter()
        .find(|album| album.name == "Photos")
        .expect("getting master photo album")
        .key_list
        .len();

    info!("Image map count is {master_image_map_count}");
    info!("Master album count is {master_album_image_count}");
}

pub(crate) fn log_duplicates(album_data: &AlbumData) {
    let mut hashes: HashMap<u32, Vec<String>> = HashMap::new();

    album_data.master_image_map.iter().for_each(|(key, image)| {
        if let Some(hash) = image.hash {
            if let Some(key_list) = hashes.get_mut(&hash) {
                key_list.push(key.clone());
            } else {
                let key_list: Vec<String> = vec![key.clone()];
                hashes.insert(hash, key_list);
            }
        }
    });

    let unique_count = hashes.iter().fold(
        0,
        |acc, (_, key_list)| {
            if key_list.len() == 1 {
                acc + 1
            } else {
                acc
            }
        },
    );

    info!("Unique image map count is {unique_count}");

    let duplicates: HashMap<u32, Vec<String>> = hashes
        .into_iter()
        .filter(|(_, key_list)| key_list.len() > 1)
        .collect();

    duplicates.iter().for_each(|(hash, key_list)| {
        let mut details = String::new();
        writeln!(details, "Duplicates for {hash}:").unwrap();
        key_list.iter().for_each(|key| {
            let image = album_data.master_image_map.get(key).unwrap();
            writeln!(details, "{}", image.image_path).unwrap();
        });
        info!("{}", details);
    })
}
