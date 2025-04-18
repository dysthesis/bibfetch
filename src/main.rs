use crate::cli::Args;

mod cli;

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;
    println!("Found command-line arguments {args:?}");
    Ok(())
}
