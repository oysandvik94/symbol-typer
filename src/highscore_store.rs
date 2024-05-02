use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use dirs::data_local_dir;

static GAME_DIR: &str = "symbol_typer";
static HIGHSCORE_FILE: &str = "highscore";

pub fn store_highscore(highscore: u32) -> io::Result<()> {
    let file_path = get_highscore_filepath()?;

    // Write or update the high score
    let mut file = File::create(file_path)?;
    file.write_all(highscore.to_string().as_bytes())?;

    Ok(())
}

pub fn retrieve_highscore() -> io::Result<u32> {
    let file_path = get_highscore_filepath()?;

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path.clone())?;

    let highscore_file_content = fs::read_to_string(file_path)?;
    let highscore = highscore_file_content.parse::<u32>().unwrap_or(0);

    Ok(highscore)
}

fn get_highscore_filepath() -> Result<PathBuf, io::Error> {
    let mut data_path = data_local_dir().unwrap();
    data_path.push(GAME_DIR);
    if !data_path.exists() {
        fs::create_dir_all(&data_path)?;
    }
    let mut file_path = PathBuf::from(&data_path);
    file_path.push(HIGHSCORE_FILE);
    Ok(file_path)
}
