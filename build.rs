use std::{env, error::Error, fs, io::ErrorKind, path::PathBuf};

use alpacker::pack::TarZstPack;
use alpacker_packer::{AssetsBuilder, PackBuilder, transform::OxipngTransform};

fn main() -> Result<(), Box<dyn Error>> {
    let profile = env::var("PROFILE")?; // e.g. "release" or "debug"
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("target"));

    let mut target_path = PathBuf::from(target_dir);
    target_path.push(profile);

    match fs::remove_file(target_path.join("manifest.json")) {
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        result => result?,
    };
    match fs::remove_file(target_path.join("content.tar.zst")) {
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        result => result?,
    };

    let pack = PackBuilder::new("content")?
        .copy_from("assets/")?
        .transform(&mut OxipngTransform::default())?;

    AssetsBuilder::new(target_path, "./")?
        .add_pack::<TarZstPack>("content", &pack)?
        .write_manifest()?;

    Ok(())
}
