use std::path::{Path, PathBuf};

use ck_snes_asm::Rom;
use pico_args::Arguments;

fn main() -> anyhow::Result<()> {
    let mut args = Arguments::from_env();
    match args.subcommand()?.as_deref() {
        None => {
            print_help();
            Ok(())
        }
        Some("decompile") => decompile(args),
        Some(other) => {
            println!("Unknown sub-command: {other}");
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("TODO: add help information");
}

fn decompile(mut args: Arguments) -> anyhow::Result<()> {
    let input: PathBuf = args.free_from_str()?;
    let output: PathBuf = args.free_from_str()?;
    run_decompile(&input, &output)
}

fn run_decompile(input: &Path, output: &Path) -> anyhow::Result<()> {
    let bytes = std::fs::read(input)?;
    Rom::new(bytes).decompile(&mut std::fs::File::create(output)?)
}
