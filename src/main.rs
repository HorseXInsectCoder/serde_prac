use serde::{Serialize, Deserialize};
use serde_json::Number;

// #[derive(Serialize, Deserialize, Debug)]
// struct Pay {
//     amount: i32,
//     tax_percent: f32,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct Person {
//     name: String,
//     age: u8,
//     phones: String,
//     pays: Vec<Pay>,
// }


fn main() {
    let json_data = std::fs::read_to_string("./data.json").unwrap();
    // 必须指定类型
    let mut data: serde_json::Value = serde_json::from_str(json_data.as_str()).unwrap();
    println!("data: {}", data);

    // 给读到的数据加一个字段
    data["car"] = serde_json::Value::String("fd".to_string());

    // 插入一个json
    let mut map_value = serde_json::Map::new();
    map_value.insert("color".to_string(), serde_json::Value::String("blue".to_string()));

    // 加数组
    map_value.insert("year".to_string(), serde_json::Value::Array(
        vec![serde_json::Value::String("1900".to_string()),
             serde_json::Value::String("2000".to_string())])
    );
    data["car_detail"] = serde_json::Value::Object(map_value);

    // 修改原字段的值
    data["age"] = serde_json::Value::Number(Number::from(20));

   println!("data: {}", data);
}
