# The ReadMe of Baby Fuzzer

## 0x01 How to Use

This fuzzer is constructed by following [this article](https://aflplus.plus/libafl-book/baby_fuzzer/baby_fuzzer.html).

Just execute the command `cargo run` under the `baby_fuzzer` directory and then you can find a *solution* test case under the `crashes` directory.

## 0x02 Warning

In the dependencies of `Cargo.toml`, the path must point to the `libafl` directory under the `LibAFL` directory (the cloned repo). If you use `libafl = "*"` to get the LibAFL dependencies, some unresolved import errors may occur like `libafl::bolts::AsSlice`.
