use std::{env, error::Error, fs::File, path::PathBuf};

use alpacker::pack::TarZstPack;
use alpacker_packer::{PackBuilder, transform::OxipngTransform};

fn main() -> Result<(), Box<dyn Error>> {
    let pack = PackBuilder::new("content")?
         .copy_from("assets/")?
         .transform(&mut OxipngTransform::default())?;

    let mut out_dir = PathBuf::from(env::var("OUT_DIR")?);
    out_dir.push("content.tar.zst");

    let mut file = File::create(out_dir)?;
    pack.make_pack::<TarZstPack>(&mut file)?;

    Ok(())
}
