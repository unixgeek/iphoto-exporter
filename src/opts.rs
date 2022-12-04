use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub(crate) struct Opts {
    #[structopt(parse(from_os_str), long = "iphoto-library-plist", short = "i")]
    pub(crate) iphoto_library_plist: PathBuf,
    #[structopt(parse(from_os_str), long = "album-export", short = "o")]
    pub(crate) album_export: PathBuf,
    #[structopt(long = "dry-run", short = "n")]
    pub(crate) dry_run: bool,
    #[structopt(long = "album", short = "a")]
    pub(crate) album: Option<String>,
}

impl Opts {
    pub(crate) fn new() -> Self {
        Opts::from_args()
    }
}
