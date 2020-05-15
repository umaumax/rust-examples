# rust lib for cpp

```
#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}
```

```
$ nm ./target/debug/libdouble_input_lib.so | grep "T" | grep double_input
0000000000047110 T double_input
```

`#[no_mangle]`を付加しない場合には、`ライブラリ名::関数名::ハッシュ`がmanglingされた状態のシンボル名となる
```
$ nm ./target/debug/libdouble_input_lib.so | grep "T" | grep double_input
0000000000047140 T _ZN16double_input_lib12double_input17had632793ea3e6b72E
$ nm ./target/debug/libdouble_input_lib.so | grep "T" | grep double_input | c++filt
0000000000047140 T double_input_lib::double_input::had632793ea3e6b72
```


## 生成されるライブラリファイル
* `Cargo.toml``[lib][rate-type]` == `["dylib"]` or `["cdylib"]`
  * `./target/debug/lib`+`Cargo.toml``[lib][name]`+`.so`
* `Cargo.toml``[lib][rate-type]` == `["staticlib"]`
  * `./target/debug/lib`+`Cargo.toml``[lib][name]`+`.a`
* `Cargo.toml``[lib][rate-type]` == `["rlib"]`(default)
  * `./target/debug/lib`+`Cargo.toml``[lib][name]`+`.rlib`

## `cdylib` と `dylib`
* `cdylib`はコンパクトなサイズのlibを作成する
  * `nm`の`N`が除去(rustのメタデータがなくなる)
  * rustの内部で利用する関数(e.g. `rust_oom`,`rust_panic`)がexportされなくなる(`T`->`t`)
    * その他、不用な関数が削除される

* 下記の方法で検証すると新規に追加されるシンボルはないことがわかる
```
command diff -c <(cat dylib-nm-output.txt | sed 's/^[0-9a-f]* *//g') <(cat cdylib-nm-output.txt | sed 's/^[0-9a-f]* *//g') | grep '^+'
```

以上より、基本的には生成した`.so`ライブラリをrustで利用しない場合は`cdylib`とした方がコンパクトなライブラリとなる

[Rust の crate\_type をまとめてみた \- Qiita]( https://qiita.com/etoilevi/items/4bd4c5b726e41f5a6689 )
