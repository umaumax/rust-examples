// lib.rsでmain.rsで見える範囲を設定する
// main.rsでmode_ex::animalが見えるようになる
pub mod animal;
pub mod utils;

pub mod internal {
    pub fn hoge() {}
}

pub use crate::internal::hoge;
