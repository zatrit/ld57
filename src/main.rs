use std::{env::current_exe, error::Error};

use alpacker::{Assets, Pack, pack::TarZstPack};

fn main() -> Result<(), Box<dyn Error>> {
    let mut binary_path = current_exe()?;
    binary_path.pop();

    let assets = Assets::load_from_dir(binary_path)?;
    let mut content = assets.load_pack::<TarZstPack>("content")?;

    print!("{}", content.get::<String>("hello_world.txt")?);

    Ok(())
}
