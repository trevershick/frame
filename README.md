Frame
=====

This is a super simple CLI that takes a duration like '24y' and returns `[<start timestamp> <end timestamp(now)> <seconds>]`.  It's useful to me and probably no one else.  I initially wrote it in python but I used it as a vehicle one day to try to learn more Rust.
That being said it's probably pretty poor rust but it was good practice with some basic Traits.


```
# frame --help

frame 1.0.0
Convert a duration specifier to a frame

USAGE:
    frame <duration>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <duration>    A duration specifier in the format <number>s|m|h|d|w|y', ex. 37h or 3d'
```
