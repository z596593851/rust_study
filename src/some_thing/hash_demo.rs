use std::collections::HashMap;

#[test]
fn hash_test() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    //let score = scores.get(&team_name).unwrap_or_else();

    let score = scores.get(&team_name).copied().unwrap_or(0);
    let score2 = scores.entry(team_name).or_insert(0);
    *score2 += 1;
    println!("score is {score}");
    println!("score2 is {score2}");
    println!("scores is {:?}",scores)
}

