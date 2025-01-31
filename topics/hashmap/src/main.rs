use std::collections::HashMap;
fn main() {
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
}
