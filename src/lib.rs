pub mod error;
use error::Error;
pub mod ioops;
use ioops::{create_and_write_archive, Manifest};
pub mod runtime;
use runtime::Operation;
use std::fs::File;

pub fn run(operation: Operation) -> Result<(), Error> {
    match operation {
        Operation::Backup(profile_path, archive_file_name) => {
            let profile_path = std::path::Path::new(profile_path);
            let mut profile = Manifest::new(profile_path)?;
            let iterator = profile.files()?;
            let output_handle = File::create(archive_file_name)?;
            create_and_write_archive(iterator, &output_handle, profile_path)?;
            Ok(())
        }
    }
}
