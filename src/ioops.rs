use crate::error::Error;
use std::cell::RefCell;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tar::Builder;

pub struct ManifestIterator {
    paths: Vec<PathBuf>,
}

// move
impl IntoIterator for ManifestIterator {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}

// ref
impl<'a> IntoIterator for &'a ManifestIterator {
    type Item = &'a PathBuf;
    type IntoIter = std::slice::Iter<'a, PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.iter()
    }
}

// mut ref
impl<'a> IntoIterator for &'a mut ManifestIterator {
    type Item = &'a mut PathBuf;
    type IntoIter = std::slice::IterMut<'a, PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.iter_mut()
    }
}

pub struct Manifest<'a> {
    path: &'a Path,
}

impl<'a> Manifest<'a> {
    pub fn new(maybe_path: &'a Path) -> Result<Manifest, Error> {
        if !maybe_path.is_dir() {
            return Err(Error::PathNotADir(maybe_path.to_string_lossy().to_string()));
        }
        Ok(Manifest { path: maybe_path })
    }
    pub fn files(&mut self) -> Result<ManifestIterator, Error> {
        let rc = RefCell::new(Vec::<PathBuf>::new());
        let mut working_path = PathBuf::new();
        working_path.push(self.path);
        Manifest::visit_paths(&working_path, &rc)?;
        Ok(ManifestIterator { paths: rc.take() })
    }
    fn visit_paths(path: &'a Path, rc: &RefCell<Vec<PathBuf>>) -> Result<(), Error> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    Manifest::visit_paths(&path, rc)?;
                } else {
                    let mut paths = rc.borrow_mut();
                    paths.push(path);
                }
            }
        }
        Ok(())
    }
}

pub fn create_and_write_archive<I>(
    iterator: I,
    archive_file: &File,
    profile_path: &Path,
) -> Result<(), Error>
where
    I: IntoIterator<Item = PathBuf>,
{
    let mut archive = Builder::new(archive_file);
    for path in iterator {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(error) => return Err(Error::OpenFile(error, path.to_string_lossy().to_string())),
        };
        let archive_file_name = match path.strip_prefix(profile_path) {
            Ok(archive_file_name) => archive_file_name,
            Err(e) => return Err(Error::ArchiveFileName(e, path)),
        };
        archive.append_file(archive_file_name, &mut file)?;
    }
    archive.finish()?;
    Ok(())
}
