use std::fs;
use std::io;

pub fn read_head() -> io::Result<usize> {
    let contents = fs::read_to_string(".cotask/HEAD")?;
    let number = contents.trim().parse::<usize>()
        .expect("HEAD file is corrupted");
    Ok(number)
}

pub fn write_head(new_head: usize) -> io::Result<()> {
    fs::write(".cotask/HEAD", new_head.to_string())?;
    Ok(())
}
