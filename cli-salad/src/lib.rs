use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<&'static str> {
    let mut fruits = vec![
        "Apple",
        "Banana",
        "Strawberry Tree Berry",
        "Pineapple",
        "Mango",
        "Peach",
        "Orange",
        "Grape",
        "Cherry",
        "Kiwi",
        "Blueberry",
    ];

    // Shuffle the fruits

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);
    // Return the first `num_fruits` fruits
    fruits.into_iter().take(num_fruits).collect()
}
