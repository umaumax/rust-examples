#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tarou = Person {
        name: "nanoha".to_string(),
        age: 10,
    };
    let json = serde_json::to_string(&tarou)?;
    println!("{}", json);

    Ok(())
}
