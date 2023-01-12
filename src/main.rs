use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Pay {
    amount: i32,
    tax_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: String,
    pays: Vec<Pay>,
}


fn main() {
    let ps = vec![Person {
        name: "voy".to_string(),
        age: 18,
        phones: "152".to_string(),
        pays: vec![
            Pay {
                amount: 100,
                tax_percent: 0.1,
            },
            Pay {
                amount: 100,
                tax_percent: 0.1,
            }],
    }];
    let json_str = serde_json::to_string(&ps).unwrap();
    let yaml_str = serde_yaml::to_string(&ps).unwrap();
}
