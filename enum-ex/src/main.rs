#[macro_use]
extern crate trait_enum;

struct Cat {
    name: String,
}
impl Cat {
    fn meow(&self) -> &'static str {
        "meow"
    }
}
struct Dog {
    name: String,
}
impl Dog {
    fn bark(&self) -> &'static str {
        "bark"
    }
}
trait HasName {
    fn get_name(&self) -> &String;
}
impl HasName for Cat {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl HasName for Dog {
    fn get_name(&self) -> &String {
        &self.name
    }
}

trait_enum! {
enum Animal:HasName {
    Cat,
    Dog,
}
}
// NOTE: trait_enum! generates below code
// enum Animal {
// Cat(Cat),
// Dog(Dog),
// }
// impl HasName for Animal {
// fn get_name(&self) -> &String {
// match self {
// Animal::Cat(cat) => cat.get_name(),
// Animal::Dog(dog) => dog.get_name(),
// }
// }
// }
impl Animal {
    pub fn get_cat(self) -> Option<Cat> {
        match self {
            Animal::Cat(cat) => Some(cat),
            _ => None,
        }
    }
    pub fn get_dog(self) -> Option<Dog> {
        match self {
            Animal::Dog(dog) => Some(dog),
            _ => None,
        }
    }
}
fn main() {
    let cat = Animal::Cat(Cat {
        name: String::from("mike"),
    });
    let dog = Animal::Dog(Dog {
        name: String::from("pochi"),
    });
    println!("cat name: {}", cat.get_name());
    println!("dog name: {}", dog.get_name());
    println!("cat say: {}", cat.get_cat().unwrap().meow());
    println!("dog say: {}", dog.get_dog().unwrap().bark());
}
