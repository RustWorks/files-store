use async_std::fs::DirEntry;
use async_std::prelude::*;
use async_std::task;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod errors;

use crate::cli::Cli;
use crate::errors::AppError;

async fn sync(folder: PathBuf) -> Result<(), AppError> {
    println!("Sync {:?}", folder);

    let mut dir = async_std::fs::read_dir(folder).await?;
    while let Some(res) = dir.next().await {
        let entry: DirEntry = res?;
        let file_type = entry.file_type().await?;

        if file_type.is_file() {
            println!("Upload {:#?}", entry);
        } else if file_type.is_dir() {
            println!("Create dir {:#?}", entry);
        }
    }
    Ok(())
}

fn main() {
    let opt = Cli::from_args();

    let result = match opt {
        Cli::Sync { folder, token: _ } => task::block_on(sync(folder)),
    };

    println!("Result {:?}", result);
}
