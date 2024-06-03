use anyhow::Result;
use files::read::read_file;

fn main() -> anyhow::Result<()> {
    let contents = read_file("foo.txt")?;

    Ok(())
}
