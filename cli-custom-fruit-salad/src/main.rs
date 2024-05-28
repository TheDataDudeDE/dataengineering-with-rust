use clap::Parser;
use cli_custom_fruit_salad::create_fruit_salad;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name")]

struct Opts {
    #[clap(short = 'f', long = "fruits")]
    fruits: Option<String>,
    #[clap(short = 'c', long = "csvfile")]
    csvfile: Option<String>,
    #[clap(short = 'd', long = "dressing")]
    dressing: Option<String>,
}

//Function that converts a csv-file to a vector of strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}


//Display the fruit salad
fn display_fruit_salad(fruits: Vec<String>, dressing: String) {
    if fruits.len() > 1{
        println!("Your fruit salad contains: ");
        for fruit in &fruits {
            println!("{}", fruit);
        }
            println!("With delicious dressing {}", dressing)
    }
    else {
        println!("You need to add fruits to your salad");
    }
}

//Add dressing
fn random_dressing() -> String {
    let dressings = vec!["honey", "maple syrup", "cinnamon", "nutmeg"];
    let mut rng = thread_rng();
    dressings.choose(&mut rng).unwrap().to_string()
}

fn main() {
    let opts = Opts::parse();
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename)
            .expect("Could not read file");
            csv_to_vec(&fruits)

        },
        None => {
            opts
            .fruits
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
        },
    };

    let dressing = opts.dressing.unwrap_or_else(|| random_dressing());
    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad, dressing);
}
