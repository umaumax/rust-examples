# getenv

RustとC++での`getenv`の速度比較

* 実測するとRustの方がC++よりも，10倍ほど遅い
* `getenv()`の使用時は競合状態を避ける必要があることからロックが利用されている
  * [CON33\-C\. ライブラリ関数の使用時は競合状態を避ける]( https://www.jpcert.or.jp/sc-rules/c-con33-c.html )
* `cerrno`はスレッドごとに独立しているので，問題ないはず
* Rustの`getenv()`の返り値は`std::string::String`であり，変更可能なようにコピーされている
  * 対して，C++の場合はコピーされない

[Man page of GETENV]( https://linuxjm.osdn.jp/html/LDP_man-pages/man3/secure_getenv.3.html )

> 通常の実装では、 getenv() は環境リスト内の文字列へのポインターを返す。 呼び出し元はこの文字列を変更しないように注意しなければならない。 この文字列を変更すると、そのプロセスの環境を変化させることになるからである。

[ENV34\-C\. getenv\(\) が返す文字列へのポインタを保存しない]( https://www.jpcert.or.jp/sc-rules/c-env34-c.html )

速度低下の考えられる理由は文字列コピーコスト?

``` bash
$ g++ -std=c++11 -lpthread main.cpp -o main -O3
0.0171603sec # getenv
0.0211518sec # getenv with pthread_mutex_lock/unlock
$ cargo run
0.217487845 sec
```
