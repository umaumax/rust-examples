# rust-examples

[The Rust Programming Language \- The Rust Programming Language]( https://doc.rust-lang.org/book/ )

## libs
* stdin/stdout/stderrがttyかどうか
  * [atty \- crates\.io: Rust Package Registry]( https://crates.io/crates/atty )
* [ansi\_term \- crates\.io: Rust Package Registry]( https://crates.io/crates/ansi_term )
  * `clap`で利用している
* [Rust のコマンドラインオプション解析色々 \- にっき]( http://ubnt-intrepid.hatenablog.com/entry/rust_commandline_parsers )
  * `clap`は`zsh`用の補完スプリクトを生成できるが，多機能すぎる
    * [clap \- Rust]( https://docs.rs/clap/2.27.1/clap/#quick-example )
      * オプションを取らない複数の引数の例は`INPUT`を参照

## memo
`rustfmt`では，分割できない長すぎる行(max_width default: 100)があるときには，挙動がおかしくなる(exit codeは`0`だが，実質formatされない模様)
下記相当のことを行えば回避することは可能な模様
```
rustfmt src/main.rs --emit stdout --config format_strings=true
rustfmt src/main.rs --emit stdout --config max_width=200
```
