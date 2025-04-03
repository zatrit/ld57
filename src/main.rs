use alpacker::{Assets, Pack, pack::TarZstPack};

fn main() {
    let mut assets = Assets::load_from_dir("./");
    if assets.is_err() && cfg!(debug_assertions) {
        assets = Assets::load_from_dir("./target/debug");
    }

    let assets = assets.unwrap();
    let mut content = assets.load_pack::<TarZstPack>("content").unwrap();

    print!("{}", content.get::<String>("hello_world.txt").unwrap());
}
