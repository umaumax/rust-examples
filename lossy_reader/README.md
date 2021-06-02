# what is this
非UTF-8文字を含むテキストに対して`read_line`を可能とする`std::io::BufRead`のラッパーの例

## how to run

```
$ cargo run -- broken-utf-8.txt
This is broken UTF-8 text.
⭐⭐⭐
NANOHA
��NA⭐N⭐O⭐N⭐A
```
