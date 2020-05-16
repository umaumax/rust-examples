# fakeroot

```
cargo build
```

```
LD_PRELOAD=./target/debug/libfakeroot.so id
# darwin
DYLD_FORCE_FLAT_NAMESPACE=1 DYLD_INSERT_LIBRARIES=./target/debug/libfakeroot.dylib id
```
