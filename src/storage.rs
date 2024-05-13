use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{constants::STORAGE_FILE_PATH, types::Storage};

pub fn write_storage_to_file(storage: &Storage) -> anyhow::Result<()> {
    let text = toml::to_string(storage)?;
    let mut file = File::create(STORAGE_FILE_PATH)?;

    file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn read_storage_from_file() -> anyhow::Result<Storage> {
    let mut file = File::open(STORAGE_FILE_PATH)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    toml::from_str(&contents).map_err(anyhow::Error::new)
}
