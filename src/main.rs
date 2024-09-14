
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use figment::{providers::Format, Figment};
use serde::Deserialize;

mod util;
mod deploy;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    profile: Option<String>,
    
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Deploy {
        #[arg(short, long, default_value_t = ToOwned::to_owned("./pond.toml"))]
        manifest: String,
    }
}

#[derive(Debug, Deserialize)]
struct Manifest {
    #[allow(unused)]
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
        .map(PathBuf::from)
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
    
    let manifest_location = match args.command {
        Commands::Deploy { manifest } => {
            manifest
        }
    };
    
    let config = read_config(&args.profile).expect("Could not find configuration");

    deploy::deploy(manifest_location, config);
}

