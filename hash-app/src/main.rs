use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

static GLOBAL_VARIABLE: i8 = 1;
static mut GLOBAL_VARIABLE_MUT: i8 = 1;

fn main() {
    ex_unsafe_call();
    ex_lazy_static_ex();
}
fn ex_unsafe_call() {
    println!("{}", GLOBAL_VARIABLE);
    // You need unsafe block for access mutable variable without mutex
    unsafe {
        println!("{}", GLOBAL_VARIABLE_MUT);
    }
}

#[derive(Debug)]
pub struct Setting {
    input_file: String,
    verbose: bool,
    level: i32,
}

lazy_static! {
    pub static ref STATIC_IMUTABLE_HASH_MAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("nanoha", 1);
        m.insert("fate", 2);
        m.insert("hayate", 3);
        m
    };
    pub static ref SETTING: Mutex<Setting> = Mutex::new(Setting {
        input_file: String::new(),
        level: 0,
        verbose: false,
    });
}

fn ex_lazy_static_ex() {
    let ref m = STATIC_IMUTABLE_HASH_MAP;
    let keys = vec!["nanoha", "fate", "hayate"];
    for key in keys {
        println!("{}:{}", key, m.get(key).unwrap());
    }

    let mut setting = SETTING.lock().unwrap();
    setting.input_file = String::from("input.txt");
    setting.level = 1;
    setting.verbose = true;
    println!("{:?}", setting);
}
