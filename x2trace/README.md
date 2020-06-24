# x2trace

ltrace,straceなどの出力結果を`trace.json`形式に変換する

マルチスレッドで各スレッドがどのような関数を呼び出しているのかをわかりやすく可視化したいという目的

## how to use
open `chrome://tracing` and drop output file

## memo
* rustの公式ツールを見ると，`trace.json`の出力に`serde_json`を利用している
  * [measureme/main\.rs at master · rust\-lang/measureme]( https://github.com/rust-lang/measureme/blob/master/crox/src/main.rs )
* [パフォーマンス計測に困らない！tracing活用術100 \- Qiita]( https://qiita.com/keishi/items/5f1af0851901e9021488 )
  * `trace.json`のGUIツールの使い方

## trace.jsonの仕様
[Trace Event Format \- Google Docs]( https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview# )

`trace.json`は下記の2種類

JSON Array Format
```
[...]
```

JSON Object Format
```
{
	"traceEvents": [...],
  ...
}
```

JSON Object Formatの`traceEvents`がJSON Array Formatと一致することもあり，基本的にはJSON Array Formatで問題はないと思われる

### Async Events
* 特に，"n"の意味がわからない
* idでまとめられるので，意味のある塊で見やすくする?
* flow eventsとは異なり，threadはまたがない

### Flow Events
* 指定したPID1,TID1上の__存在する__sliceから指定したPID2,TID2上の__存在する__sliceへの経路である
* s(start)とt(end)で同一のidを指定すること

### Metadata Events
設定したいメタデータごとに下記のjsonを作成する
```
{
 "name": "thread_name", "ph": "M", "pid": 2343, "tid": 2347,
 "args": {
  "name" : "VALUE"
 }
}
```

## ./x2trace.awk
### ltrace
```
seq 1 10 | ltrace -ttt -T -f -o ltrace-ttt-T-f-o.xargs.log xargs -L1 -I{} -P 4 bash -c "sleep 1 & echo {}"
cat ltrace-ttt-T-f-o.xargs.log | ./x2trace.awk > xargs.json

# with system call
seq 1 10 | ltrace -S -ttt -T -f -o ltrace-S-ttt-T-f-o.xargs.log xargs -L1 -I{} -P 4 bash -c "sleep 1 & echo {}"
cat ltrace-S-ttt-T-f-o.xargs.log | ./x2trace.awk > xargs.json
```

* 共有ライブラリの同一の関数内で再帰呼び出しがないことが前提の処理
  * ltraceでindent付きで記録している場合はそのindentから区別することは可能
* ltraceのtsはおそらく，呼び出されたときの時間であると思われる
  * resumedのts - durがunfinishedのtsと一致するわけではない
* `-e '*'`とするとライブラリファイル名付きで出力され，関数名が不明のものはアドレスで示される
* 標準出力とファイル出力(`-o`)で出力formatが微妙に異なる
* ltraceの影響で上記のプログラムは相当処理が遅くなっている

### strace
```
seq 1 10 | strace -ttt -T -f -q -o strace-ttt-T-f-q-o.xargs.log xargs -L1 -I{} -P 4 bash -c "sleep 1 & echo {}"
cat strace-ttt-T-f-q-o.xargs.log | ./x2trace.awk > xargs.json
```
