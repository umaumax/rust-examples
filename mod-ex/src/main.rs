use mod_ex::utils;
// 下記はエラーとなる
// use crate::utils;

// 下記は同じ効果に見える
// mod animal;
use mod_ex::animal;

fn main() {
    // 自身のクレート名::でlib.rsからアクセスできる範囲を参照できる
    // pub useしているので、modを省略して参照可能
    mod_ex::hoge();
    mod_ex::internal::hoge();

    crate::animal::hello();
    animal::hello();
    animal::cat::meow();
    mod_ex::animal::hello();
    mod_ex::animal::cat::meow();

    mod_ex::utils::hello();
    utils::hello();
    let _ = utils::get_animal_data();
}
