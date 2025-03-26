use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Luke Skywalker"),
        age: 24,
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json_str);

    // Deserialzed
    let deserialzed_person: Person = serde_json::from_str(&json_str).unwrap();
    println!("Deserialized JSON: {:?}", deserialzed_person);


    // Serialize to YAML
    let yaml_str = serde_yaml::to_string(&person).unwrap();
    println!("Serialzed YAML: {}", yaml_str);

    // Deserialzed
    let deserialzed_person2: Person = serde_yaml::from_str(&yaml_str).unwrap();
    println!("Deserialzed YAML: {:?}", deserialzed_person2);


    // Serialize to TOML
    let toml_str = toml::to_string(&person).unwrap();
    println!("Serialzed TOML: {}", toml_str);

    // Deserialzed
    let deserialzed_person3: Person = toml::from_str(&toml_str).unwrap();
    println!("Deserialzed TOML: {:?}", deserialzed_person3);

}