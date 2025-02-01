use std::collections::HashMap;
fn main() {
<<<<<<< HEAD
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Sunface1", 198);
    scores.insert("Sunface2", 928);
    scores.insert("Sunface3", 938);

    println!("{:?}", scores);

    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98)); // or
    assert_eq!(score, Some(98).as_ref());

    if scores.contains_key("Sunface1") {
        let score = scores["Sunface1"];
        assert_eq!(score, 198);
        scores.remove("Sunface1");
    }

    assert_eq!(scores.len(), 3);
    println!("{:?}", scores);
=======
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



>>>>>>> 797dbcb (Exercise 3 solved)
}
