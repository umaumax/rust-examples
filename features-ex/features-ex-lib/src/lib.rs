#[cfg(feature = "hoge")]
pub fn hoge() {
    println!("hoge");
}

#[cfg(feature = "fuga")]
pub fn fuga() {
    println!("fuga");
}

#[cfg(feature = "piyo")]
pub fn piyo() {
    println!("piyo");
}
