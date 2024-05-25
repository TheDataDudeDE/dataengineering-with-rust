use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    // Create a Vector Deque of fruits
    let mut fruit_salad = VecDeque::new();
    fruit_salad.push_back("Apple");
    fruit_salad.push_front("Banana");
    fruit_salad.push_back("Cherry");
    fruit_salad.push_front("Date");
    fruit_salad.push_back("Elderberry");
    fruit_salad.push_front("Fig");
    fruit_salad.push_back("Grape");

    // Shuffle the fruit salad
    let mut rng = thread_rng();
    fruit_salad.make_contiguous().shuffle(&mut rng);


    // Print the fruit salad
    println!("Fruit Salad with:");
    for fruit in fruit_salad {
        println!("\t{}", fruit);
    }

}
