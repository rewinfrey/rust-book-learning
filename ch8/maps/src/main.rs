/*
HashMaps are stored on the heap, just like other dynamic collections or types.

HashMaps are homogenous, all keys must be of same type, and all values must be of same type.
*/

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let blue_team = String::from("Blue");
    let red_team = String::from("Red");

    // Because blue_team and red_team are Strings, they are moved to the scores HashMap.
    // This means blue_team and red_team are no longer valid references after they are inserted into the HashMap.
    // Other values that implement the Copy trait would be copied to the HashMap on insertion.
    scores.insert(blue_team, 10);
    scores.insert(red_team, 5);

    // Cannot reference blue_team here:
    // println!("{}", blue_team);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Red")];

    let initial_scores = vec![10, 50];

    let mut iter_scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", iter_scores);

    // Accessing values from a HashMap.
    let score = iter_scores.get(&String::from("Blue"));

    println!("{:?}", score);

    // Can iterate over all keys / values in a HashMap:
    for (key, value) in &iter_scores {
        println!("{}: {}", key, value);
    }

    // Overwriting values in a HashMap:
    iter_scores.insert(String::from("Blue"), 100);

    println!("{:?}", iter_scores);

    // Conditional insertion if there is not an existing value:
    iter_scores.entry(String::from("Orange")).or_insert(75);
    iter_scores.entry(String::from("Blue")).or_insert(1000);

    println!("{:?}", iter_scores);
}
