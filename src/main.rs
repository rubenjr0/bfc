use std::{fs::File, io::Write, path::PathBuf, process::Command};

use anyhow::Result;

mod codegen;
mod parser;
mod tokenizer;

use clap::Parser;
use codegen::CodeGen;
use tokenizer::Tokenizer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_path: PathBuf,
    #[arg(short, help = "The default is the same as input's filename")]
    output_path: Option<PathBuf>,
    #[arg(short, long, default_value_t = false, help = "Don't print messages")]
    quiet: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Run the program after compilation"
    )]
    run: bool,
}

fn main() -> Result<()> {
    let Args {
        input_path,
        output_path,
        quiet,
        run,
    } = Args::parse();

    let tokenizer = Tokenizer::new(&input_path)?;

    let parser = parser::Parser::new(tokenizer);

    if !quiet {
        println!("Compiling....");
    }
    let mut codegen = CodeGen::new(parser);
    let code = codegen.gen();

    let output_path = output_path.unwrap_or(input_path.with_extension("s"));
    let output_path = output_path.with_extension("s");
    let object_path = output_path.with_extension("o");
    let object_path = object_path.to_str().unwrap();
    let exec_path = output_path.with_extension("");
    let exec_path = exec_path.to_str().unwrap();
    let output_path = output_path.to_str().unwrap();

    if !quiet {
        println!("Writing......");
    }
    let mut output_file = File::create(&output_path)?;
    for instruction in &code {
        write!(output_file, "{instruction}\n")?;
    }

    if !quiet {
        println!("Assembling...");
    }
    let nasm = Command::new("nasm")
        .args(["-f", "elf32"])
        .arg(output_path)
        .status()
        .expect("Could not run NASM, is it properly installed?");

    if !nasm.success() {
        anyhow::bail!("NASM exited with status {nasm}");
    }

    if !quiet {
        println!("Linking......");
    }
    let ld = Command::new("ld")
        .args(["-m", "elf_i386"])
        .args(["-o", exec_path])
        .arg(object_path)
        .status()
        .expect("Could not run `ld`, is it properly installed?");

    if !ld.success() {
        anyhow::bail!("LD exited with status {ld}");
    }

    std::fs::remove_file(object_path)?;

    if !quiet {
        println!("Done!");
    }

    if run {
        if !quiet {
            println!("Running...");
        }
        Command::new(format!("./{exec_path}")).status()?;
    }
    Ok(())
}
