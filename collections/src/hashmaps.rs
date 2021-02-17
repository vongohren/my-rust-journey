use std::collections::HashMap;

pub fn main() {
    

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Scores are: {:#?}", scores);


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    // un referenced field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("References hashmap {:#?}", map);
    println!("String {}", field_name);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // returns an option and as a developer I need to handle that
    println!("Score {:#?}", score.unwrap());

    println!("Scores iterated over:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Green")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    
}