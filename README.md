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

e.g.
```
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



----

## memo
`rustfmt`では，分割できない長すぎる行(max_width default: 100)があるときには，挙動がおかしくなる(exit codeは`0`だが，実質formatされない模様)

下記相当のことを行えば回避することは可能な模様
```
rustfmt src/main.rs --emit stdout --config format_strings=true
rustfmt src/main.rs --emit stdout --config max_width=200
```
