
//Function to remove a fruit from a fruit list
fn remove_fruit<'a>(fruit_list: Vec<&'a str>, fruit_to_remove: &'a str) -> Vec<&'a str> {
    fruit_list.into_iter().filter(|&fruit| fruit != fruit_to_remove).collect()
}

fn sort_fruit<'a>(fruit_list: Vec<&'a str>) -> Vec<&'a str> {
    let mut fruits = fruit_list;
    fruits.sort();
    fruits
}



fn main() {
    let mut fruit_list = vec!["apple", "banana", "pear", "kiwi", "orange"];
    println!("Fruit list: {:?}", fruit_list);
    fruit_list.push("grape");
    println!("Fruit list after push: {:?}", fruit_list);
    let last = fruit_list.pop();
    println!("Last fruit that has been dropped: {:?}", last);
    println!("Fruit list after pop: {:?}", fruit_list);

    
    //Removing a fruit from the list
    let fruit_to_remove = "banana";
    let fruits = remove_fruit(fruit_list, fruit_to_remove);
    println!("Fruit list after removing {:?}", fruits);
    let fruits = sort_fruit(fruits);
    println!("Fruit list after removing and sorting {:?}", fruits);


}
