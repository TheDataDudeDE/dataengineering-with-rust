use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(version = "1.0", author = "Abdullah")]

struct Opts {
    #[clap(short, long, default_value = "3")]
    num_fruits: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let fruits = create_fruit_salad(opts.num_fruits);
    println!(
        "Created a fruit salad with {} fruits: {:?}",
        num_fruits,
        create_fruit_salad(num_fruits)
    );
}
