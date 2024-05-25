use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruits = LinkedList::new();
    fruits.push_back("Apple");
    fruits.push_back("Banana");
    fruits.push_back("Strawberry Tree Berry");

    // Shuffle the fruits
    let mut rng = thread_rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);
    let mut fruits: LinkedList<_> = fruits.into_iter().collect();
    fruits.push_front("Pineapple");
    fruits.push_back("Mango");
    fruits.push_back("Peach");

    // Print the fruits
    println!("Fruit salad:");
    for fruit in fruits.iter() {
        println!("{}", fruit);
    }
}
