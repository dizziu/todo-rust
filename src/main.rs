use clap::Parser;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(subcommand)]
    commands: Option<Actions>,
}

#[derive(clap::Subcommand, Debug)]
enum Actions {
    #[clap(about = "Adds entry to todo list")]
    Add { entry: Option<Vec<String>> },
    #[clap(about = "Removes entry to todo list")]
    Remove { id: Option<u32> },
    #[clap(about = "Lists todo entry")]
    List,
}

fn add(mut file: File, c: &Vec<String>) {
    let v = c.iter().map(|e| e.to_string() + "\n").collect::<Vec<_>>();

    v.iter().for_each(|f| {
        file.write(f.as_bytes()).expect("writing failed");
    });
    println!("Added Successfully")
}

fn remove(file: File, id: u32) {
    let mut temp_file = File::create("temp_file.txt").unwrap();
    BufReader::new(file)
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(f, e)| {
            let f = f as u32 + 1;
            if f != id {
                writeln!(temp_file, "{}", e.unwrap()).unwrap();
            }
        });
    fs::rename("temp_file.txt", "todo.txt").unwrap();
    println!("Successfully removed");
}

fn list(file: File) {
    println!("Todo List:");
    BufReader::new(file)
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(f, e)| {
            println!("{}.{}", f + 1, e.unwrap());
        });
}

fn main() {
    let file = OpenOptions::new()
        .append(true)
        .read(true)
        .open("todo.txt")
        .expect("connot open file");

    let args = Args::parse();
    match args.commands {
        Some(x) => match x {
            Actions::Add { entry: Some(ref c) } => {
                add(file, c);
            }
            Actions::Remove { id: Some(id) } => {
                remove(file, id);
            }
            Actions::List => list(file),
            _ => println!("Enter a value"),
        },
        _ => println!("Error"),
    }
}
