use clap::{arg, Parser, Subcommand};
use serde::{Deserialize};
use std::{fs, io};

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Comme,
}

#[derive(Subcommand, Debug)]
enum Comme {
    Add {
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        price: f64,
    },
    Delete {
        #[arg(short, long)]
        target: u32,
    },
    List,
    Summary {
        #[arg(short, long)]
        month: Option<u32>,
    },
}

#[derive(Debug, Deserialize)]
struct Track {
    id: u32,
    date: String,
    description: String,
    price: f64,  // Changed to f64 for consistency with `Comme::Add`
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Comme::Add { description, price } => {
            println!("one element added: {} - {}", description, price);
            // Here, you could add logic to save this new element to the JSON file.
        }
        Comme::Delete { target } => {
            println!("one element deleted: {:?}", target);
            // Here, you could add logic to remove an element by id from the JSON file.
        }
        Comme::List => {
            println!("showing list:");
            if let Err(e) = list() {
                eprintln!("Error reading the list: {}", e);
            }
        }
        Comme::Summary { month } => match month {
            Some(number) => println!("call fn summary for month: {}", number),
            None => println!("call fn summary for all months"),
        },
    }
}

fn list() -> io::Result<()> {
    let file = fs::read_to_string("Data.json")?;
    let expenses: Vec<Track> = serde_json::from_str(&file)?;

    println!("id    date            description     price:");
    for expense in expenses {
        println!("{}     {}      {}           {}", expense.id , expense.date ,expense.description , expense.price);
    }
    Ok(())
}
