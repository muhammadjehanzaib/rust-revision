use std::collections::HashMap;
fn main() {
    // Exercise 1
    // let mut scores = HashMap::new();
    // scores.insert("Sunface", 98);
    // scores.insert("Daniel", 95);
    // scores.insert("Ashley", 69);
    // scores.insert("Katie", 58);

    // // Get returns an Option<&V>
    // let score = scores.get("Sunface");
    // assert_eq!(score, Some(98).as_ref());

    // if scores.contains_key("Daniel") {
    //     // Indexing returns a value V
    //     let score = scores["Daniel"];
    //     assert_eq!(score, 95);
    //     scores.remove("Daniel");
    // }

    // assert_eq!(scores.len(), 3);

    // for (name, score) in scores {
    //     println!("The score of {} is {}", name, score);
    // }

    // Exercise 2

    // let teams = [
    //     ("chines team", 100),
    //     ("Americans team", 10),
    //     ("France team", 50),
    // ];

    // let mut teams_map1 = HashMap::new();

    // for team in &teams {
    //     teams_map1.insert(team.0, team.1);
    // }

    // // let teams_map2 = HashMap::from(teams); // method 1
    // let teams_map2 = teams.into_iter().collect(); // method 2
    // assert_eq!(teams_map1,teams_map2);
    // println!("Success");

    // Exercise 3

    let mut player_state = HashMap::new();
    player_state.entry("health").or_insert(100);

    assert_eq!(player_state["health"], 100);

    player_state.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_state["health"], 100);

    let health: &mut u8 = player_state.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);
    println!("Success!");

}

fn random_stat_buff() -> u8 {
    42
}
