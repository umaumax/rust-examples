# rust-examples

[The Rust Programming Language \- The Rust Programming Language]( https://doc.rust-lang.org/book/ )

## how to install
e.g.
```
cargo install --git https://github.com/umaumax/rust-examples rcat
```

## libs memo
* stdin/stdout/stderrがttyかどうか
  * [atty \- crates\.io: Rust Package Registry]( https://crates.io/crates/atty )
* [ansi\_term \- crates\.io: Rust Package Registry]( https://crates.io/crates/ansi_term )
  * `clap`で利用している
* [Rust のコマンドラインオプション解析色々 \- にっき]( http://ubnt-intrepid.hatenablog.com/entry/rust_commandline_parsers )
  * `clap`は`zsh`用の補完スプリクトを生成できるが，多機能すぎる
    * [clap \- Rust]( https://docs.rs/clap/2.27.1/clap/#quick-example )
      * オプションを取らない複数の引数の例は`INPUT`を参照

### enum
[strum \- crates\.io: Rust Package Registry]( https://crates.io/crates/strum )

```
cargo add strum
cargo add strum_macros
```

```
#[derive(strum_macros::EnumString)]
#[strum(serialize_all = "kebab_case")] // default is camel case
enum ColorWhen {
    Always,
    Never,
    Auto,
}
```

とすることで，下記相当の実装となる

```
impl FromStr for ColorWhen {
    type Err = ColorWhenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(ColorWhen::Always),
            "never" => Ok(ColorWhen::Never),
            "auto" => Ok(ColorWhen::Auto),
            _ => Err(ColorWhenError(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct ColorWhenError(String);
impl fmt::Display for ColorWhenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ColorWhen string is 'always|never|auto', not allowed '{}'",
            self
        )
    }
}
impl std::error::Error for ColorWhenError {}
```

[Derive EnumString · Peternator7/strum Wiki]( https://github.com/Peternator7/strum/wiki/Derive-EnumString )

### error handling
概念としては，[Nicer error reporting \- Command Line Applications in Rust]( https://rust-cli.github.io/book/tutorial/errors.html )のページがわかりやすいが，
__注意点として，`failure`はメンテナンスされていない([From failure to Fehler]( https://boats.gitlab.io/blog/post/failure-to-fehler/ ))__

とりあえず，下記のいずれかを使ってみる(どちらを使ったほうが良いかは，各ページ下部の`Comparison to XXX`に説明がある)

* [anyhow \- crates\.io: Rust Package Registry]( https://crates.io/crates/anyhow )
  * 独自のエラー型にこだわらない場合(簡単に利用できるので，おすすめ)
* [thiserror \- crates\.io: Rust Package Registry]( https://crates.io/crates/thiserror )
  * 独自のエラー型にこだわる場合

独自のエラー型の実装としては，下記のページが参考になる

[std::error::Error \- Rust]( https://doc.rust-lang.org/std/error/trait.Error.html#examples )

独自のエラー型を外部ライブラリを利用しないで定義する場合

e.g.
```rust
use std::fmt;

#[derive(Debug)]
struct XXXError(String);

impl Error for XXXError {}
impl fmt::Display for XXXError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XXXError occurred!: {}", self)
    }
}
```

`thiserror`を利用した場合との比較はこちら[Rustの便利クレート \- Qiita]( https://qiita.com/qryxip/items/7c16ab9ef3072c1d7199#thiserror )

`anyhow`の使い方は[Refactor: use error handling lib 'anyhow' · umaumax/rust\-examples@37540cc]( https://github.com/umaumax/rust-examples/commit/37540ccf1422833371fd01171467a4eda4577d21 )を参照

`one-off`(一回限り)のエラーに関しては`anyhow!`を利用すると良い

そのときは，下記の記述が必要

```
#[macro_use]
extern crate anyhow;
```

ライブラリを利用しない場合は[Rustのエラーとなかよくなる \- 3c1uのブログ]( https://3c1u.hatenablog.com/entry/2019/09/18/060000 )のページがわかりやすい

まとめとしては，
[Best error\-handling practices \- Jan 2020 : rust]( https://www.reddit.com/r/rust/comments/ej67aa/best_errorhandling_practices_jan_2020/ )
の意見に賛同

サーベイはこちら
[Error Handling Survey]( https://blog.yoshuawuyts.com/error-handling-survey/ )

----

## memo
* `rustfmt`では，分割できない長すぎる行(max_width default: 100を超える)のとき，exit codeは`0`だが，formatされない現象に遭遇する
  * [Gives up on chains if any line is too long\. · Issue \#3863 · rust\-lang/rustfmt]( https://github.com/rust-lang/rustfmt/issues/3863 )
  * [RustFmt fails to format file with long line within scope of lambda passed to member function · Issue \#3135 · rust\-lang/rustfmt]( https://github.com/rust-lang/rustfmt/issues/3135 )

下記相当のことを行えば回避できる
```bash
rustfmt src/main.rs --emit stdout --config format_strings=true
rustfmt src/main.rs --emit stdout --config max_width=200
```

----
----
----
----

## memo
* [フィールドと変数が同名の時にフィールド初期化省略記法を使う
]( https://doc.rust-jp.rs/book/second-edition/ch05-01-defining-structs.html#a%E3%83%95%E3%82%A3%E3%83%BC%E3%83%AB%E3%83%89%E3%81%A8%E5%A4%89%E6%95%B0%E3%81%8C%E5%90%8C%E5%90%8D%E3%81%AE%E6%99%82%E3%81%AB%E3%83%95%E3%82%A3%E3%83%BC%E3%83%AB%E3%83%89%E5%88%9D%E6%9C%9F%E5%8C%96%E7%9C%81%E7%95%A5%E8%A8%98%E6%B3%95%E3%82%92%E4%BD%BF%E3%81%86 )
* [構造体更新記法で他のインスタンスからインスタンスを生成する
]( https://doc.rust-jp.rs/book/second-edition/ch05-01-defining-structs.html#a%E6%A7%8B%E9%80%A0%E4%BD%93%E6%9B%B4%E6%96%B0%E8%A8%98%E6%B3%95%E3%81%A7%E4%BB%96%E3%81%AE%E3%82%A4%E3%83%B3%E3%82%B9%E3%82%BF%E3%83%B3%E3%82%B9%E3%81%8B%E3%82%89%E3%82%A4%E3%83%B3%E3%82%B9%E3%82%BF%E3%83%B3%E3%82%B9%E3%82%92%E7%94%9F%E6%88%90%E3%81%99%E3%82%8B )
* [異なる型を生成する名前付きフィールドのないタプル構造体を使用する
]( https://doc.rust-jp.rs/book/second-edition/ch05-01-defining-structs.html#a%E7%95%B0%E3%81%AA%E3%82%8B%E5%9E%8B%E3%82%92%E7%94%9F%E6%88%90%E3%81%99%E3%82%8B%E5%90%8D%E5%89%8D%E4%BB%98%E3%81%8D%E3%83%95%E3%82%A3%E3%83%BC%E3%83%AB%E3%83%89%E3%81%AE%E3%81%AA%E3%81%84%E3%82%BF%E3%83%97%E3%83%AB%E6%A7%8B%E9%80%A0%E4%BD%93%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%99%E3%82%8B )
* [ジェネリクスを使用したコードのパフォーマンス]( https://doc.rust-jp.rs/book/second-edition/ch10-01-syntax.html#a%E3%82%B8%E3%82%A7%E3%83%8D%E3%83%AA%E3%82%AF%E3%82%B9%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%97%E3%81%9F%E3%82%B3%E3%83%BC%E3%83%89%E3%81%AE%E3%83%91%E3%83%95%E3%82%A9%E3%83%BC%E3%83%9E%E3%83%B3%E3%82%B9 )
* [注釈: 違いはあるものの、トレイトは他の言語でよくインターフェイスと呼ばれる機能に類似しています。]( https://doc.rust-jp.rs/book/second-edition/ch10-02-traits.html )
* 同じメソッドのオーバーライドした実装からは、デフォルト実装を呼び出すことができないことに注意
  * いわゆるsuper的なことはできない? <- そもそも，c++でこれできる?
* [Freezing \- Rust By Example]( https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/freeze.html )
  * 最近のrustが賢いのか普通に{}なしでもOKの模様...
* `PartialOrd`: `Ord`は`order`の略称
* [args関数と不正なユニコード]( https://doc.rust-jp.rs/book/second-edition/ch12-01-accepting-command-line-arguments.html#args%E9%96%A2%E6%95%B0%E3%81%A8%E4%B8%8D%E6%AD%A3%E3%81%AA%E3%83%A6%E3%83%8B%E3%82%B3%E3%83%BC%E3%83%89 )

* [Rc<T>は、参照カウント方式のスマートポインタ \- The Rust Programming Language]( https://doc.rust-jp.rs/book/second-edition/ch15-04-rc.html )
> Rc::clone(&a)ではなく、a.clone()を呼ぶこともできますが、Rustのしきたりは、この場合Rc::cloneを使うことです。 Rc::cloneの実装は、多くの型のclone実装のように、全てのデータのディープコピーをすることではありません。 Rc::cloneの呼び出しは、参照カウントをインクリメントするだけであり、時間はかかりません。 データのディープコピーは時間がかかることもあります。参照カウントにRc::cloneを使うことで、 視覚的にディープコピーをする類のクローンと参照カウントを増やす種類のクローンを区別することができます。 コード内でパフォーマンスの問題を探す際、ディープコピーのクローンだけを考慮し、Rc::cloneの呼び出しを無視できるのです。

* [循環参照は、メモリをリークすることもある]( https://doc.rust-jp.rs/book/second-edition/ch15-06-reference-cycles.html )

* [トレイトオブジェクトには、オブジェクト安全性が必要]( https://doc.rust-jp.rs/book/second-edition/ch17-02-trait-objects.html#a%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%E3%81%AB%E3%81%AF%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%E5%AE%89%E5%85%A8%E6%80%A7%E3%81%8C%E5%BF%85%E8%A6%81 )

* 関数内で関数を定義できる(文末のセミコロン不要)
  * ただし，変数のキャプチャは行われない
* iterに対する`.zip()`は少ない方の要素数に調整される
  * 片方がNoneを返したら，zipはNoneを返す
* コンストラクタは通例として，`new()`メソッドを利用する
* [構造体を分配する]( https://doc.rust-jp.rs/book/second-edition/ch18-03-pattern-syntax.html#a%E6%A7%8B%E9%80%A0%E4%BD%93%E3%82%92%E5%88%86%E9%85%8D%E3%81%99%E3%82%8B )
  * 同様に，Enumや参照も分配可能
* `_`は値を束縛しない特殊な変数名である(`_xxx`などはwarningが省略されるようになるだけであり，通常の変数のように値を束縛することになる)
* match式
  * 複数の条件に合致するパターンでは先に記述されたパターンのみマッチする
  * `@束縛`なるものがあり，値を保持する変数を生成するのと同時にその値がパターンに一致するかを調べることができる

* [C\-like \- Rust By Example]( https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html )
  * enumの宣言時にi32の具体的な値を入れられるが，他の値はだめ
  * [Enums with constant values in Rust \- Stack Overflow]( https://stackoverflow.com/questions/36928569/enums-with-constant-values-in-rust )
これが答えかな?
* [User Shepmaster \- Stack Overflow]( https://stackoverflow.com/users/155423/shepmaster )
  * この人がrustの質問によく出てくる
* [Nesting and labels \- Rust By Example]( https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html )
  * loopにラベルを付加して，指定したラベルのloopを抜けることができる
* [Returning from loops \- Rust By Example]( https://doc.rust-lang.org/rust-by-example/flow_control/loop/return.html )
  * loopブロックは値を返すことができる
* [for and range \- Rust By Example]( https://doc.rust-lang.org/rust-by-example/flow_control/for.html#for-and-iterators )
  * `for`で回すときの`iter`/`into_iter`/`iter_mut`の比較
* ifは式なので，3項演算子のように利用できる
  * このとき，`;`を忘れないように
* matchは式でもある
  * `let x = match y {}`の中でreturnやpanicをするケースでは各条件の中の返り値を省略できる

* クロージャーを直接関数の引数にする場合には，クロージャの引数の型を省略できるが，それ以外の場合には明示する必要があることに注意
  * [rust \- Expected bound lifetime parameter, found concrete lifetime \[E0271\] \- Stack Overflow]( https://stackoverflow.com/questions/31362206/expected-bound-lifetime-parameter-found-concrete-lifetime-e0271 )

* [main関数の返り値の型]( https://doc.rust-lang.org/reference/crates-and-source-files.html#main-functions )

* 構造体のdefault value
  * [Rustの構造体に対する疑問を検証してみた \- 雑なメモ書き]( https://hiroyukim.hatenablog.com/entry/2020/02/22/205032 )
  * [Rustのstd::default::Default]( https://www.utam0k.jp/blog/2018/05/28/rust_std_default/ )

----

## enum
* あらゆる型を指定可能
* メソッドを定義可能

## 式
`{文; 式}`は`式`である

## match
* switchのdefaultに当たるものは`_`

## ファイル名
下記は特殊なファイル名として扱われる

* main.rs
* lib.rs
* mod.rs
  * このファイルはRust 2018では，わざわざ作らなくても良い [Path clarity \- The Edition Guide]( https://doc.rust-lang.org/nightly/edition-guide/rust-2018/module-system/path-clarity.html#no-more-modrs )
* build.rs

## 注意点
* 関数の最後に`return`するときに誤って，`;`をつけてしまいがち

## tips
* `let a: () = vec![];`とすると，右辺の型がerrorメッセージとして出現するので便利

## web site
* 標準ドキュメント
  * [標準ドキュメント検索]( https://doc.rust-lang.org/std/?search= )
* ライブラリ
  * [crates\.io: Rust Package Registry]( https://crates.io/ )
  * [Docs\.rs]( https://docs.rs/ )
* [Rust Playground]( https://play.rust-lang.org/ )

* 正規表現
  * [Redux regex playground]( https://2fd.github.io/rust-regex-playground/#method=find&regex=%5Cw%2B&text=abc )

* 命名規則
  * [Naming \- Rust API Guidelines]( https://rust-lang.github.io/api-guidelines/naming.html )

> In UpperCamelCase, acronyms and contractions of compound words count as one word: use Uuid rather than UUID, Usize rather than USize or Stdin rather than StdIn. 

* エラーコード一覧
  * [Rust Compiler Error Index]( https://doc.rust-lang.org/error-index.html )

## ライフタイム注釈
* `'a`: `a`でなくてもよい
* `'static`: これは特別扱いされ，プログラム全体の期間を示す

```
// グローバルな領域にある文字列の参照をしているため，&strとしては，このブロック外からもアクセス可能
// 丁寧に考えると右記 let literal_str: &'static str = "xyz";
{ let literal_str = "xyz"; }

// こちらの場合はブロック内のスコープでのみ有効なので，ブロックを抜けると参照できなくなる
{ let string = String::from("xyz"); }
```

## test

```
#[cfg(test)]
mod tests {
	// 外部モジュールで定義したもの全てがこのtestsモジュールでも使用可能になるようにする
    use super::*;

    #[test]
    fn xxx_test() {

    }
}
```

> 二つの値が等しいとアサーションを行う関数の引数は、 expectedとactualと呼ばれ、引数を指定する順序が問題になる言語やテストフレームワークもあることに注意してください。 ですがRustでは、leftとrightと呼ばれ、期待する値とテスト下のコードが生成する値を指定する順序は、 問題になりません。

とあるが，推奨する順番は?

> testsモジュールの`#[cfg(test)]`という注釈は、コンパイラにcargo buildを走らせた時ではなく、cargo testを走らせた時にだけ、 テストコードをコンパイルし走らせるよう指示します。これにより、ライブラリをビルドしたいだけの時にはコンパイルタイムを節約し、 テストが含まれないので、コンパイル後の成果物のサイズも節約します。

> `tests`ディレクトリのサブディレクトリ内のファイルは個別クレートとしてコンパイルされたり、 テスト出力に区域が表示されることがないため，テストで共通に利用する仕組みは`tests/common/mod.rs`に記述すると良い

## for_each

* `for_each`では`continue`や`break`はできないので，それを行いたい場合は`for` loopもしくは`try_for_each`を利用すること
  * [std::iter::Iterator - for_each - Rust]( https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each )
  * [std::iter::Iterator - try_for_each - Rust]( https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_for_each )

## Crates.io

> 公開は永久なので、クレートの公開時には気をつけてください。バージョンは絶対に上書きできず、 コードも削除できません。crates.ioの一つの主な目標が、 crates.ioのクレートに依存している全てのプロジェクトのビルドが、 動き続けるようにコードの永久アーカイブとして機能することなのです。バージョン削除を可能にしてしまうと、 その目標を達成するのが不可能になってしまいます。ですが、公開できるクレートバージョンの数に制限はありません。

## modules

* [Rustのモジュールを詳細に理解する\(1\) モジュール入門 \(この回は簡単です！\) \- 簡潔なQ]( https://qnighy.hatenablog.com/entry/2019/05/06/190000 )

> Rust2018時代のイディオマティックなRustコードでは、
> extern crate を使いません。
> 代わりに、extern prelude (1.30.0で安定化) を使います。

参照: [Path clarity \- The Edition Guide]( https://doc.rust-lang.org/nightly/edition-guide/rust-2018/module-system/path-clarity.html#no-more-extern-crate )

* ダウンロード数ランキング
  * [Crates \- crates\.io: Rust Package Registry]( https://crates.io/crates?sort=downloads )

* [Rust のプレリュード \(prelude\) とは何か \- ひだまりソケットは壊れない]( https://vividcode.hatenablog.com/entry/rust/what-is-prelude )

* プログレスバー
  * [indicatif \- crates\.io: Rust Package Registry]( https://crates.io/crates/indicatif )
* ロガー
  * [env\_logger \- crates\.io: Rust Package Registry]( https://crates.io/crates/env_logger )
* [Signal handling \- Command Line Applications in Rust]( https://rust-cli.github.io/book/in-depth/signals.html )
* [exitcode \- crates\.io: Rust Package Registry]( https://crates.io/crates/exitcode )

> System exit code constants as defined by sysexits.h

## arm cross build

[rustをインストールしてhello world をARM用にクロスコンパイルするまでの手順 \- Qiita]( https://qiita.com/tetsu_koba/items/1ab400a3d4ec9725b044 )

```
# ターゲットの確認
# rustup target list

rustup target add armv7-unknown-linux-gnueabihf

# rustのビルド時にlinkerオプションにarmクロスコンパイラを指定する必要がある
sudo apt-get -y install g++-arm-linux-gnueabihf
```

```
$ rustc -C linker=arm-linux-gnueabihf-gcc --target=armv7-unknown-linux-gnueabihf hello_world.rs
$ file hello_world
hello_world: ELF 32-bit LSB shared object, ARM, EABI5 version 1 (SYSV), dynamically linked, interpreter /lib/ld-, for GNU/Linux 3.2.0, BuildID[sha1]=35c6ee7d36b72e877eb9094890a4c661e55e3421, not stripped

# 2.6MB => 161KB
$ arm-linux-gnueabihf-strip hello_world
hello_world: ELF 32-bit LSB shared object, ARM, EABI5 version 1 (SYSV), dynamically linked, interpreter /lib/ld-, for GNU/Linux 3.2.0, BuildID[sha1]=35c6ee7d36b72e877eb9094890a4c661e55e3421, stripped
```

### cargoでcross buildしたい場合の設定
`~/.cargo/config`を新規に作成すること
```
cat >> ~/.cargo/config << EOF
# Set default build target to armv7hf
# [build]
# target = "armv7-unknown-linux-gnueabihf"

[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
```

```
$ cargo build --target armv7-unknown-linux-gnueabihf
$ ls ./target/armv7-unknown-linux-gnueabihf/debug/hello_world
```

====

# rustからcppを呼び出す手順

[rust\-ffi\-examples/rust\-to\-cpp at master · sn99/rust\-ffi\-examples]( https://github.com/sn99/rust-ffi-examples/tree/master/rust-to-cpp )

1. cppで`extern "C"`として、関数のシンボルがデマングルされるようにする

```
extern "C"
int triple_input(int input) {
    return input * 3;
}
```

2. rust側でcppの関数を呼びだすためのプロトタイプ宣言相当のものを記述し、`unsafe`で呼び出す

```
extern crate libc;

extern {
    fn triple_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 4;
    let output = unsafe { triple_input(input) };
    println!("{} * 3 = {}", input, output);
}
```

3. 何かしらのライブラリ(e.g. `cc`)を利用して、`build.rs`を作成して、ビルドできるようにする

`build.rs`
```
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/triple.cpp")
        .cpp(true)
        .compile("libtriple.a");
}
```

c++のライブラリはパッケージ名に適当なハッシュ関数が付加されたディレクトリに配置されている
`./target/debug/build/rust-to-cpp-4bcf80914f0d03a5/out/libtriple.a`

利用されたコマンドは`output`ファイルを見ればわかる
```
$ cat ./target/debug/build/rust-to-cpp-4bcf80914f0d03a5/output | grep "^running"
running: "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m64" "-Wall" "-Wextra" "-o" "xxx/target/debug/build/rust-to-cpp-4bcf80914f0d03a5/out/src/triple.o" "-c" "src/triple.cpp"
running: "ar" "crs" "xxx/target/debug/build/rust-to-cpp-4bcf80914f0d03a5/out/libtriple.a" "xxx/target/debug/build/rust-to-cpp-4bcf80914f0d03a5/out/src/triple.o"
```

また、ビルド方法として、`cmake`を採用している場合には次の例のように`cmake`クレートを利用すればよい

[rust\-ffi\-examples/rust\-to\-cmake at master · sn99/rust\-ffi\-examples]( https://github.com/sn99/rust-ffi-examples/tree/master/rust-to-cmake )

====

## bindgen

`clang++`が必須

[rust\-lang/rust\-bindgen: Automatically generates Rust FFI bindings to C \(and some C\+\+\) libraries\.]( https://github.com/rust-lang/rust-bindgen )

[Requirements \- The \`bindgen\` User Guide]( https://rust-lang.github.io/rust-bindgen/requirements.html )

> If you are generating bindings to C++, you almost definitely want 3.9 or greater.

### `bzip2`をwrapするチュートリアル

[Tutorial \- The \`bindgen\` User Guide]( https://rust-lang.github.io/rust-bindgen/tutorial-0.html )

[fitzgen/bindgen\-tutorial\-bzip2\-sys: A tutorial/example crate for generating C/C\+\+ bindings on\-the\-fly with libbindgen]( https://github.com/fitzgen/bindgen-tutorial-bzip2-sys )

```
git clone https://github.com/fitzgen/bindgen-tutorial-bzip2-sys.git
cd bindgen-tutorial-bzip2-sys
cargo build
cargo test
```

`as _`は、型キャストの`auto`版だと思われる

## bindgenコマンド
* 対象とするファイルが`.h`の場合はcのファイルと解釈するので、`.hpp`するとcppのファイルとして解釈するようになる
  * もしくは`bindgen xxx.h -o xxx.rs -- -x c++`とする
* `bindgen xxx.h -o xxx.rs -- -Ixxx`とすることで、ヘッダのインクルードの解決ができる
