use std::{env, fs, io::ErrorKind, path::PathBuf};

use alpacker::pack::TarZstPack;
use alpacker_packer::{AssetsBuilder, PackBuilder, transform::OxipngTransform};

fn main() {
    let profile = env::var("PROFILE").unwrap(); // e.g. "release" or "debug"
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("target"));

    let mut target_path = PathBuf::from(target_dir);
    target_path.push(profile);

    match fs::remove_file(target_path.join("manifest.json")) {
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        result => result.unwrap(),
    };
    match fs::remove_file(target_path.join("content.tar.zst")) {
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        result => result.unwrap(),
    };

    let pack = PackBuilder::new("content")
        .unwrap()
        .copy_from("assets/")
        .unwrap()
        .transform(&mut OxipngTransform::default())
        .unwrap();

    AssetsBuilder::new(target_path, "./")
        .unwrap()
        .add_pack::<TarZstPack>("content", &pack)
        .unwrap()
        .write_manifest()
        .unwrap();
}
