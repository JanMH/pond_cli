use std::{
    fs::File, io::Read, os::unix::fs::MetadataExt, path::{Path, PathBuf}
};

use clap::Parser;
use figment::{providers::Format, Figment};
use reqwest::blocking::{multipart::{Form, Part}, Client};
use serde::Deserialize;

mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = ToOwned::to_owned("./pond.toml"))]
    manifest: String,

    #[arg(short, long)]
    profile: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    name: String,
    artifact: String,
}

impl Manifest {
    fn artifact_location(&self, manifest_path: impl AsRef<Path>) -> PathBuf {
        let path: &Path = self.artifact.as_ref();
        if path.is_absolute() {
            return path.to_owned();
        }
        match manifest_path.as_ref().parent() {
            Some(p) => p.join(path),
            None => path.to_owned(),
        }
    }
}

#[derive(Deserialize)]
struct GlobalConfig {
    host: String,
    access_token: String,
}

fn read_config(profile: &Option<String>) -> anyhow::Result<GlobalConfig> {
    use figment::providers::Toml;
    let config_path = option_env!("CONFIG_PATH")
        .map(|d| PathBuf::from(d))
        .or_else(|| dirs::home_dir().map(|h| h.join(".pond.toml")))
        .ok_or(anyhow::Error::msg("Could not identify correct config path"))?;
    let mut figment = Figment::from(Toml::file(&config_path).nested());
    if let Some(profile) = profile {
        figment = figment.select(profile);
    }
    Ok(figment.extract()?)
}

fn main() {
    let args = Args::parse();
    let config = read_config(&args.profile).expect("Could not find configuration");

    let mut manifest_file = File::open(&args.manifest).unwrap();
    let mut manifest_string = String::new();
    manifest_file
        .read_to_string(&mut manifest_string)
        .expect("Failed to read manifest");

    let manifest: Manifest = toml::from_str(&manifest_string).unwrap();
    let location = manifest.artifact_location(&args.manifest);
    println!("Compressing {}", location.to_string_lossy());
    let artifact = util::zip_dir_to_tmp(&location).expect("Failed to compress file");
    println!("total size {}",File::open(&artifact).unwrap().metadata().unwrap().size() );
    
    let artifact_part = Part::file(&artifact).unwrap().mime_str("application/octet-stream");

    let form = Form::new()
        .part("manifest", Part::text(manifest_string).mime_str("application/yaml").unwrap())
        .part("artifact", artifact_part.unwrap());

    let client = Client::new();
    let mut response = client
        .post(config.host + "/deploy")
        .multipart(form)
        .bearer_auth(config.access_token)
        .send()
        .unwrap();
    println!("Returned status {}", response.status());
    
    std::io::copy(&mut response, &mut std::io::stdout()).expect("Failed to read server response");

    std::fs::remove_file(&artifact).unwrap();
}
