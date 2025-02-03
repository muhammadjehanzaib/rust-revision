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

    // let mut player_state = HashMap::new();
    // player_state.entry("health").or_insert(100);

    // assert_eq!(player_state["health"], 100);

    // player_state.entry("health").or_insert_with(random_stat_buff);
    // assert_eq!(player_state["health"], 100);

    // let health: &mut u8 = player_state.entry("health").or_insert(50);
    // assert_eq!(health, &100);
    // *health -= 50;
    // assert_eq!(*health, 50);
    // println!("Success!");

    // Exercise 4
    // let viking = HashMap::from([
    //     (Viking::new("Einar", "Norway"), 25),
    //     (Viking::new("Olaf", "Denmark"), 24),
    //     (Viking::new("Harald", "Iceland"), 12),
    // ]);

    // for (vikin, health) in &viking {
    //     println!("{:?} has {} hp", vikin, health);
    // }

    // Exercise 5
    // let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    // map.insert(1, 2);
    // map.insert(3, 4);

    // assert!(map.capacity() >= 100);

    // map.shrink_to(50);
    // assert!(map.capacity() >= 50);

    // map.shrink_to_fit();
    // assert!(map.capacity() >= 2);
    // println!("Success");

    // Exercsie 6
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    // Ownership moved here
    m2.insert(&v2, v1);

    assert_eq!(v2, "hello");

    println!("Success!");
}
// #[derive(Debug, Eq, Hash, PartialEq)]
// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     fn new(name:&str, country:&str) -> Self{
//         Self{
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// fn random_stat_buff() -> u8 {
//     42
//}
