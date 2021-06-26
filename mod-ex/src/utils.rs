// 他のモジュールはcreateからたどることができる
use crate::animal;
// 階層的に下記でもよい
// use super::animal;

pub fn hello() {
    println!("utils hello");
}

pub fn get_animal_data() -> animal::Data {
    animal::Data { id: 123 }
}
