use crate::iphoto::Album;
use log::info;
use std::collections::HashSet;

const ALBUM_TYPES_TO_IGNORE: [&str; 4] = ["Book", "Shelf", "Slideshow", "Special Roll"];

pub(crate) fn filter_albums(albums: &mut Vec<Album>, album: Option<String>) {
    // Log what is being ignored.
    albums
        .iter()
        .filter(|album| {
            ALBUM_TYPES_TO_IGNORE
                .contains(&album.r#type.as_ref().unwrap_or(&"".to_string()).as_str())
        })
        .for_each(|album| info!(r#"Ignoring album "{}""#, album.name));

    // If an album name is provided, reduce list to just that album.
    if let Some(album_name) = album {
        albums.retain(|album| album.name == album_name);
    } else {
        albums.retain(|album| {
            !ALBUM_TYPES_TO_IGNORE
                .contains(&album.r#type.as_ref().unwrap_or(&"".to_string()).as_str())
        });
    }
}

pub(crate) fn handle_non_album_photos(albums: &mut Vec<Album>) {
    let mut keys_in_albums: HashSet<String> = HashSet::new();

    // Create a set that contains all image keys in an album.
    albums
        .iter()
        .filter(|album| album.name != "Photos")
        .for_each(|album| {
            album.key_list.iter().for_each(|key| {
                keys_in_albums.insert(key.clone());
            })
        });

    // Create a list of keys not in an album.
    let mut not_in_an_album_album: Vec<String> = Vec::new();
    albums
        .iter()
        .find(|album| album.name == "Photos")
        .unwrap()
        .key_list
        .iter()
        .for_each(|key| {
            if !keys_in_albums.contains(key) {
                not_in_an_album_album.push(key.clone());
            }
        });

    albums.retain(|album| album.name != "Photos");

    albums.push(Album {
        name: "Not in an Album".to_string(),
        r#type: Some("Regular".to_string()),
        key_list: not_in_an_album_album,
    })
}

#[cfg(test)]
mod tests {
    use crate::filter::{filter_albums, handle_non_album_photos};
    use crate::iphoto::Album;

    #[test]
    fn test_filter_albums() {
        let a1 = Album {
            name: "Whatever".to_string(),
            r#type: Some("Book".to_string()),
            key_list: vec![],
        };
        let a2 = Album {
            name: "Some Album".to_string(),
            r#type: None,
            key_list: vec![],
        };
        let mut albums = vec![a1, a2];

        filter_albums(&mut albums, None);

        assert_eq!(1, albums.len());
        assert_eq!("Some Album", albums[0].name);
    }

    #[test]
    fn test_filter_albums_specific_album() {
        let a1 = Album {
            name: "Whatever".to_string(),
            r#type: None,
            key_list: vec![],
        };
        let a2 = Album {
            name: "Some Album".to_string(),
            r#type: None,
            key_list: vec![],
        };
        let mut albums = vec![a1, a2];

        filter_albums(&mut albums, Some("Whatever".to_string()));

        assert_eq!(1, albums.len());
        assert_eq!("Whatever", albums[0].name);
    }

    #[test]
    fn test_handle_non_album_photos() {
        let a1 = Album {
            name: "Photos".to_string(),
            r#type: None,
            key_list: vec!["1".to_string(), "2".to_string()],
        };
        let a2 = Album {
            name: "Some Album".to_string(),
            r#type: None,
            key_list: vec!["2".to_string()],
        };

        let mut albums = vec![a1, a2];
        handle_non_album_photos(&mut albums);
        assert_eq!(2, albums.len());
        assert_eq!("Some Album".to_string(), albums[0].name);
        assert_eq!("Not in an Album".to_string(), albums[1].name);
        assert_eq!(1, albums[1].key_list.len());
        assert_eq!("1".to_string(), albums[1].key_list[0]);
    }
}
