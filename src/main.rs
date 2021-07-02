use aopacchi::*;

use std::{fs, io};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse()?;
    let mut bytes = fs::read(&args.sav_file)?;
    println!("Read in {} bytes from '{}'", bytes.len(), args.sav_file);

    patch_sprites(&mut bytes)?;

    // where is the player in the title?? :(
    // let sprite = fs::read("sprites/gfx/title/player.2bpp")?;
    // let offsets = sav_tools::search_bytes(&bytes, &sprite, &vec![]);
    // println!("offsets: {:x?}", offsets);

    Ok(())
}

fn patch_sprites(bytes: &mut [u8]) -> io::Result<()> {
    println!("Patching sprite data...");

    let orig_sprites = load_orig_sprites()?;
    let new_sprites = load_new_sprites()?;

    for (key, data) in &orig_sprites {
        assert!(new_sprites.contains_key(key));
        let new_data = new_sprites.get(key).unwrap();
        assert!(new_data.len() <= data.len());
    }

    for (name, data) in orig_sprites {
        let offsets = sav_tools::search_bytes(&bytes, &data, &vec![]);
        println!(
            "Found {} match(es) for '{}': {:x?}", offsets.len(), name, offsets);
        
        let new_data = new_sprites.get(name).unwrap();
        for offset in offsets {
            for i in 0..new_data.len() {
                bytes[offset+i] = new_data[i];
            }
        }
    }

    println!("Saving patched game to 'patched_ao.gb'");
    fs::write("patched_ao.gb", &bytes)?;

    Ok(())
}

fn load_orig_sprites() -> io::Result<HashMap<&'static str, Vec<u8>>> {
    let mut sprites = HashMap::new();

    // i cannot seem to find the player sprite for the title screen 
    // in the final rom!
    // sprites.insert("player_title", 
    //     fs::read("sprites/gfx/title/player.2bpp")?);
    sprites.insert("player_battle_front",
        fs::read("sprites/orig/gfx/player/red.pic")?);
    sprites.insert("player_battle_back",
        fs::read("sprites/orig/gfx/player/redb.pic")?);
    sprites.insert("player_sprite_main",
        fs::read("sprites/orig/gfx/sprites/red.2bpp")?);
    sprites.insert("player_sprite_bike",
        fs::read("sprites/orig/gfx/sprites/red_bike.2bpp")?);
    sprites.insert("player_fish_front",
        fs::read("sprites/orig/gfx/overworld/red_fish_front.2bpp")?);
    sprites.insert("player_fish_back",
        fs::read("sprites/orig/gfx/overworld/red_fish_back.2bpp")?);
    sprites.insert("player_fish_side",
        fs::read("sprites/orig/gfx/overworld/red_fish_side.2bpp")?);

    Ok(sprites)
}

fn load_new_sprites() -> io::Result<HashMap<&'static str, Vec<u8>>> {
    let mut sprites = HashMap::new();

    // sprites.insert("player_title", 
    //     fs::read("sprites/gfx/title/player.2bpp")?);
    sprites.insert("player_battle_front",
        fs::read("sprites/new/gfx/player/red.pic")?);
    sprites.insert("player_battle_back",
        fs::read("sprites/new/gfx/player/redb.pic")?);
    sprites.insert("player_sprite_main",
        fs::read("sprites/new/gfx/sprites/red.2bpp")?);
    sprites.insert("player_sprite_bike",
        fs::read("sprites/new/gfx/sprites/red_bike.2bpp")?);
    sprites.insert("player_fish_front",
        fs::read("sprites/new/gfx/overworld/red_fish_front.2bpp")?);
    sprites.insert("player_fish_back",
        fs::read("sprites/new/gfx/overworld/red_fish_back.2bpp")?);
    sprites.insert("player_fish_side",
        fs::read("sprites/new/gfx/overworld/red_fish_side.2bpp")?);
    
    Ok(sprites)
}
