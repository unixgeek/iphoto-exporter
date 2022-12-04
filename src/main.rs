mod exiv2;
mod export;
mod filter;
mod hash;
mod info;
mod iphoto;
mod opts;
mod progress;

use crate::opts::Opts;

fn main() {
    env_logger::init();

    let opts = Opts::new();

    println!("Parsing AlbumData");
    let mut album_data = iphoto::parse_album_data(&opts.iphoto_library_plist);

    hash::generate_image_hash("Generating Checksums", &mut album_data.master_image_map);

    info::log_photo_counts(&album_data);

    info::log_duplicates(&album_data);

    let is_album_specified = opts.album.is_some();
    filter::filter_albums(&mut album_data.albums, opts.album);

    if !is_album_specified {
        filter::handle_non_album_photos(&mut album_data.albums);
    }

    if !opts.dry_run {
        export::export_albums("Exporting Albums", album_data, &opts.album_export);
    }

    println!("Done");
}
