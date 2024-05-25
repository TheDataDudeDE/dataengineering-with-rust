use std::collections::HashMap;

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}


fn calculate_weights(languages: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    for year in languages.values_mut() {
        *year = 2024 - *year;
    }

    let mut years_active = HashMap::new();
    for (language, &year) in languages.iter() {
        years_active.insert(language.to_string(), year);
    }
    years_active
}




fn main() {
    let mut languages = init_languages();
    let years_active = calculate_weights(&mut languages);
    println!("The years active for each language is {:?}", years_active);
}
