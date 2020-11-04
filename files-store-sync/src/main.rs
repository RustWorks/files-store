use async_std::fs::DirEntry;
use async_std::prelude::*;
use async_std::task;
use files_store_domain::CreateFsNodeDirectory;
use std::path::PathBuf;
use structopt::StructOpt;

mod api;
mod cli;
mod errors;

use crate::api::Api;
use crate::cli::Cli;
use crate::errors::AppError;

async fn sync(folder: PathBuf, api: Api) -> Result<(), AppError> {
    println!("Sync {:?}", folder);

    let mut dir = async_std::fs::read_dir(folder).await?;
    while let Some(res) = dir.next().await {
        let entry: DirEntry = res?;
        let file_type = entry.file_type().await?;

        if file_type.is_file() {
            println!("Upload {:#?}", entry);
        } else if file_type.is_dir() {
            dbg!(&entry);
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let create_fs_node_directory = CreateFsNodeDirectory::new(file_name, None);
            let creted_fs_node_directory = api.create_directory(&create_fs_node_directory).await?;
            dbg!(&creted_fs_node_directory);
        }
    }
    Ok(())
}

fn main() {
    let opt = Cli::from_args();

    let result = match opt {
        Cli::Sync {
            folder,
            token,
            host,
        } => {
            let api = Api::new(token, host);
            task::block_on(sync(folder, api))
        }
    };

    println!("Result {:?}", result);
}
