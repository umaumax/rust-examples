# stop-the-world

`LD_PRELOAD` base library for stopping(send `SIGTSTP`) the program before starting `main()`

## for what?
* lazy `gdb` attach pid
* to set pid to `tasks` file of `cgroup`

## how to run
```
# linux
LD_PRELOAD=./target/debug/libstop_the_world.so ls
# darwin
DYLD_FORCE_FLAT_NAMESPACE=1 DYLD_INSERT_LIBRARIES=./target/debug/libstop_the_world.dylib ls

# environment variable
STOP_THE_WORLD_INIT=1 # (default is enable)
STOP_THE_WORLD_TERM=0 # (default is disable)
```

## how to send SIGCONT signal
```
fg
# or
kill -SIGCONT $PID
# or
pkill -SIGCONT $PNAME
```
