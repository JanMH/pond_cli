use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::{SimpleFileOptions, ZipWriter};

pub fn zip_dir_to_tmp(source: &Path) -> anyhow::Result<PathBuf> {
    let tmp_file_destination = std::env::temp_dir()
        .join("deployer_archive".to_owned()+
        &Alphanumeric.sample_string(&mut thread_rng(), 8)
        + ".zip");
    let zip_file = File::create(&tmp_file_destination)?;
    let walkdir = WalkDir::new(source);
    let mut it = walkdir
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.into_path()));

    zip_dir(&mut it, source, zip_file)?;
    Ok(tmp_file_destination)
}

fn zip_dir(
    it: &mut dyn Iterator<Item = PathBuf>,
    prefix: &Path,
    outfile: File,
) -> zip::result::ZipResult<()>
{
    let mut zip = ZipWriter::new(outfile);
    let options = SimpleFileOptions::default();

    for path in it {
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            let mut file = File::open(&path)?;
            zip.start_file(name.to_str().unwrap(), options)?;
            io::copy(&mut file, &mut zip)?;
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
