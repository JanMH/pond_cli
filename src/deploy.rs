use std::{
    fs::File, io::Read, os::unix::fs::MetadataExt
};

use reqwest::blocking::{multipart::{Form, Part}, Client};

use crate::{util, GlobalConfig, Manifest};

pub fn deploy(manifest_location: String, config: GlobalConfig) {
    let mut manifest_file = File::open(&manifest_location).unwrap();
    let mut manifest_string = String::new();
    manifest_file
        .read_to_string(&mut manifest_string)
        .expect("Failed to read manifest");

    let manifest: Manifest = toml::from_str(&manifest_string).unwrap();
    let location = manifest.artifact_location(&manifest_location);
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