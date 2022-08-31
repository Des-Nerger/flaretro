# flaretro
(Not ready for the public yet, but I keep it here anyway to hold myself accountable / motivated and as a cloud backup.)
```sh
$ cd mods
$ ln -s "/full/path/to/Diablo II Shareware v 1.04"/*.mpq/extracted/data "D2sw/MpqsData"
$ cargo build --release
$ retroarch -vL ../target/release/libflaretro.so
```
