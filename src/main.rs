use clap::{arg, Parser, Subcommand};
use serde::{Deserialize , Serialize};
use serde_json::{error, to_string_pretty, to_writer } ; 
use std::{fs, io::{self, Write}, result};
use std::fs::OpenOptions  ; 
use chrono::{prelude, NaiveDate} ;
use serde_json::to_string;

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
        id : Option<u32>
    },
    List,
    Summary {
        #[arg(short, long)]
        month: Option<String>,
    },

}

#[derive(Debug, Deserialize , Serialize)]
struct Track {
    id: u32,
    date: String,
    description: String,
    price: f64,  // Changed to f64 for consistency with `Comme::Add`
}
impl  Track {
    fn new(id :u32 , description : String  , price : f64) -> Self {
        let local_time = prelude::Local::now() ; 
        let current_date = local_time.date_naive() ; 
        Track {
            id ,
            date : current_date.to_string() , 
            description , 
            price
        }
    }
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Comme::Add { description, price } => {
              let resute =   add_to_list(description , price) ; 
              println!("{:?}" , resute)
        }
        Comme::Delete {  id  } => {
           match id  {
              Some(id) =>
               {
                let resute  = delete_by_id(id)  ;
            } ,
            None => {
                delet_all();
            }     
           }
        }
        Comme::List => {
            println!("showing list:");
            if let Err(e) = list() {
                eprintln!("Error reading the list: {}", e);
            }
        }
        Comme::Summary { month } => match month {
            Some(number) =>  {
                summary_by_month( number) ; 
            }
            None =>  {
                let result = summary() ; 
            }
        },
    }
}

fn list() -> io::Result<()> {
    let file = fs::read_to_string("Data.json")?;
    let expenses: Vec<Track> = serde_json::from_str(&file)?;
    if expenses.len() == 0 {
        println!("NO tracke existe") ; 
        return   Ok(()) ; 
    }
    println!("id    date            description     price:");
    for expense in expenses {
        println!("{}     {}      {}           {}", expense.id , expense.date ,expense.description , expense.price);
    }
    Ok(())
}



fn add_to_list(description : &String , price : &f64) -> Result<bool , std::io::Error>{
    if  price.round() <= 1.00 || description.len() < 4 {
         return  Ok(false) 
    }else {
        let file = fs::read_to_string("Data.json")?;
        let mut expenses: Vec<Track> = serde_json::from_str(&file)?;
        let id = get_id(&expenses) ; 
        let new_track = Track::new(id, description.to_string(), *price);
        expenses.push(new_track);

       
        
        let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)

        .open("Data.json")?;

    let json = to_string_pretty(&expenses)?;
    file.write_all(json.as_bytes())?; 

        
        
    }
    Ok(true)
}


fn get_id (list :&[Track]) -> u32 {
   let mut last_id : u32 =  0; 
   if list.len() == 0  {
    return  last_id ; 
   }else {
     for  element in list {
        last_id = element.id ; 
     };
     return  last_id + 1 ;
   }
   last_id 
}


fn delet_all () -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("Data.json")?; 
    let arr : Vec<Track> = vec![] ; 
    let json = to_string_pretty(&arr)?;
    file.write_all(json.as_bytes())?;  
    
    Ok(())
}

fn delete_by_id (id : &u32) -> Result<() , io::Error>  {
    let file = fs::read_to_string("Data.json")?;
    let  expenses: Vec<Track> = serde_json::from_str(&file)?;
    let mut new_list : Vec<Track> = vec![] ; 


    for  track in expenses  {
        if &track.id !=  id {
            new_list.push( track);
        }
    }

    println!("One track deleter  width id  {}" , id) ; 
    let mut file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .open("Data.json")?; 
    let json  :String = to_string_pretty(&new_list)?; 
    file.write_all(json.as_bytes() )? ; 
    Ok(())
}


fn summary () -> io::Result<()> {
    let file = fs::read_to_string("Data.json")?;
    let  expenses: Vec<Track> = serde_json::from_str(&file)?;

    calculate_the_sammary( expenses , false ) ; 

    Ok(())
}


fn calculate_the_sammary (expenses : Vec<Track> , type_of_fn   : bool)  {
    let mut summary : f64  = 0.0 ; 
    if  expenses.len() == 0 {
        println!("Can't get summary list is empty ")  ; 
    }else {
    for track in &expenses {
        summary += track.price 
    }
    if type_of_fn {
        println!("Total expenses of the month is  : ${}" , summary ) ; 
    }else {

        println!("Total expenses : ${}" , summary ) ; 
    }
    }
}


fn summary_by_month (month : &String) -> io::Result<()> { 

    let file = fs::read_to_string("Data.json")?;
    let  expenses: Vec<Track> = serde_json::from_str(&file)?;
    let resute = summary_filter_by_mother(expenses, month) ; 
    calculate_the_sammary(resute , true) ; 
    Ok(())
}

fn summary_filter_by_mother(expenses : Vec<Track> , month : &String)  ->  Vec<Track> {
    let mut  new_arr : Vec<Track> =  vec![] ; 
    for track in expenses {
        let container =  track.date.contains(month) ; 
        if container  {
            new_arr.push(track);
        }
    }
    new_arr
}

