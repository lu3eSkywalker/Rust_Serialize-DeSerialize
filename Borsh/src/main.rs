
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct MyStruct {
    id: u64,
    data: String,
    v: Vec<u32>
}

fn main() {
    let original = MyStruct {
        id: 42,
        data: String::from("Hello There"),
        v: vec![1, 2, 3]
    };

    let mut buffer: Vec<u8> = Vec::new();

    original.serialize(&mut buffer).unwrap();

    println!("Serialized data: {:?}", buffer);

    // Deserialize
    let deserialized = MyStruct::try_from_slice(&mut buffer).unwrap();
    println!("Successfully Serialized and deserialized: {:?}", deserialized);
}