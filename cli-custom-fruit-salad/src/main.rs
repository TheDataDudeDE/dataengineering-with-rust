use clap::Parser;
use cli_custom_fruit_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name")]

struct Opts {
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
}

//Function that converts a csv-file to a vector of strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}


//Display the fruit salad
fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains: ");
    for fruit in fruits {
        println!("{}", fruit);
    }
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

    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad);
}
