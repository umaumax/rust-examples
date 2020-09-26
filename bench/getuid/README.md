# getuid

* マルチスレッドでの競合を防ぐ必要がないAPIの呼び出し速度比較
  * `nix::unistd::getuid`は結局`libc::getuid()`のラッパーでその中身は
``` cpp
extern "C" {
    pub fn getuid() -> uid_t;
}
```

* [nix/unistd\.rs at master · nix\-rust/nix]( https://github.com/nix-rust/nix/blob/master/src/unistd.rs#L1290 )
* [libc/mod\.rs at master · rust\-lang/libc]( https://github.com/rust-lang/libc/blob/master/src/unix/mod.rs#L910 )

``` bash
$ g++ -std=c++11 main.cpp -o main -O3
0.417687sec
$ cargo run
0.448845752 sec
```

``` bash
$ objdump -d ./target/debug/getuid
0000000000005020 <_ZN3nix6unistd6getuid17h6161036f108c5a35E>:
    5020:       50                      push   %rax
    5021:       ff 15 19 be 23 00       callq  *0x23be19(%rip)        # 240e40 <getuid@GLIBC_2.2.5>
    5027:       89 04 24                mov    %eax,(%rsp)
    502a:       8b 04 24                mov    (%rsp),%eax
    502d:       89 44 24 04             mov    %eax,0x4(%rsp)
    5031:       8b 44 24 04             mov    0x4(%rsp),%eax
    5035:       59                      pop    %rcx
    5036:       c3                      retq
    5037:       66 0f 1f 84 00 00 00    nopw   0x0(%rax,%rax,1)
    503e:       00 00
```
