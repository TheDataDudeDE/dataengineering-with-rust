use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    //A vector of fruits
    let mut fruits = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    //Extend the fruit list with another 5 fruits
    fruits.extend(vec!["Fig", "Grape", "Honeydew", "Kiwi", "Lemon"]);
    
    //Scramble of shuffle the fruits
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);
    
    //Print out the fruit salad
    println!("Fruit salad");
    for fruit in &fruits {
        println!("In the fruit salad we have {fruit}!")
    }

}
