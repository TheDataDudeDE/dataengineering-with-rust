use clap::Parser;
use cli_custom_fruit_salad::create_fruit_salad;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::Write;

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name")]

struct Opts {
    #[clap(short = 't', long = "data_type", default_value = "fruits")]
    data_type: String,
    #[clap(short = 'i', long = "fruits")]
    items: Option<String>,
    #[clap(short = 'c', long = "csvfile")]
    csvfile: Option<String>,
    #[clap(short = 'd', long = "dressing")]
    dressing: Option<String>,
    #[clap(short = 'o', long = "output")]
    output: Option<String>,
}

enum DataType {
    Fruits,
    Books,
    Movies,
}

struct Book {
    author: String,
    title: String,
}

struct Movie {
    title: String,
    genre: String,
}

//Function that converts a csv-file to a vector of strings
fn csv_to_fruits(csv: &str) -> Vec<String> {
    csv.split(',').map(|s| s.trim().to_string()).collect()
}

fn csv_to_movies(csv: &str) -> Vec<Movie> {
    csv.lines()
        .skip(1)
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            Movie {
                title: parts[0].trim().to_string(),
                genre: parts[1].trim().to_string(),
            }
        })
        .collect()
}
fn csv_to_books(csv: &str) -> Vec<Book> {
    csv.lines()
        .skip(1)
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            Book {
                title: parts[0].trim().to_string(),
                author: parts[1].trim().to_string(),
            }
        })
        .collect()
}

//Display the fruit salad
fn display_fruit_salad(fruits: &Vec<String>, dressing: &String) {
    if fruits.len() > 1 {
        println!("Your fruit salad contains: ");
        for fruit in fruits {
            println!("{}", fruit);
        }
        println!("With delicious dressing {}", dressing)
    } else {
        println!("You need to add items (fruits, books, movies) to your list");
    }
}

fn write_fruit_salad_to_csv(
    fruits: Vec<String>,
    dressing: String,
    output: &str,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create(output)?;
    writeln!(file, "Fruit:")?;
    for fruit in fruits {
        writeln!(file, "{}", fruit)?;
    }
    writeln!(file, "Dressing: {}", dressing)?;
    Ok(())
}
fn write_books_to_csv(books: Vec<Book>, output: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(output)?;
    writeln!(file, "Author, Title")?;
    for book in books {
        writeln!(file, "{}, {}", book.title, book.author)?;
    }
    Ok(())
}
fn write_movies_to_csv(movies: Vec<Movie>, output: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(output)?;
    writeln!(file, "Title, Genre")?;
    for movie in movies {
        writeln!(file, "{}, {}", movie.title, movie.genre)?;
    }
    Ok(())
}

//Add dressing
fn random_dressing() -> String {
    let dressings = vec!["honey", "maple syrup", "cinnamon", "nutmeg"];
    let mut rng = thread_rng();
    dressings.choose(&mut rng).unwrap().to_string()
}

fn main() {
    let opts = Opts::parse();
    let data_type = match opts.data_type.as_str() {
        "fruits" => DataType::Fruits,
        "books" => DataType::Books,
        "movies" => DataType::Movies,
        _ => {
            eprintln!("Unsupported data type: {}", opts.data_type);
            return;
        }
    };
    match data_type {
        DataType::Fruits => {
            let fruit_list = match opts.csvfile {
                Some(filename) => {
                    let fruits = std::fs::read_to_string(filename).expect("Could not read file");
                    csv_to_fruits(&fruits)
                }
                None => opts
                    .items
                    .unwrap_or_default()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            };
            let dressing = opts.dressing.unwrap_or_else(|| random_dressing());
            let fruit_salad = create_fruit_salad(fruit_list);
            display_fruit_salad(&fruit_salad, &dressing);
            if let Some(output_file) = opts.output {
                write_fruit_salad_to_csv(fruit_salad, dressing, &output_file)
                    .expect("Could not write to file");
            }
        }
        DataType::Books => {
            let book_list = match opts.csvfile {
                Some(filename) => {
                    let books = std::fs::read_to_string(filename).expect("Could not read file");
                    csv_to_books(&books)
                }
                None => opts
                    .items
                    .unwrap_or_default()
                    .lines()
                    .map(|line| {
                        let parts: Vec<&str> = line.split(',').collect();
                        Book {
                            title: parts[0].trim().to_string(),
                            author: parts[1].trim().to_string(),
                        }
                    })
                    .collect(),
            };
            if let Some(output_file) = opts.output {
                write_books_to_csv(book_list, &output_file).expect("Could not write to file");
            }
        }
        DataType::Movies => {
            let movie_list = match opts.csvfile {
                Some(filename) => {
                    let movies = std::fs::read_to_string(filename).expect("Could not read file");
                    csv_to_movies(&movies)
                }
                None => opts
                    .items
                    .unwrap_or_default()
                    .lines()
                    .map(|line| {
                        let parts: Vec<&str> = line.split(',').collect();
                        Movie {
                            title: parts[0].trim().to_string(),
                            genre: parts[1].trim().to_string(),
                        }
                    })
                    .collect(),
            };
            if let Some(output_file) = opts.output {
                write_movies_to_csv(movie_list, &output_file).expect("Could not write to file");
            }
        }
    }
}
