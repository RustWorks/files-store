use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "files_store_sync", about = "Files sync")]
pub enum Cli {
    #[structopt(name = "sync", about = "Sync files")]
    Sync {
        #[structopt(short = "f", long = "folder", parse(from_os_str))]
        folder: PathBuf,

        #[structopt(short = "t", long = "token")]
        token: String,
    },
}
