# The ReadMe of Baby Fuzzer

## 0x01 How to Use

You should install AFL++ at first.

Under the `libafl_xpdf` directory, download `Xpdf 3.02`.

```Shell
cd /path/to/libafl_xpdf/
wget https://dl.xpdfreader.com/old/xpdf-3.02.tar.gz
tar xvf xpdf-3.02.tar.gz
rm xpdf-3.02.tar.gz
mv xpdf-3.02 xpdf
```

Then, execute `cargo build --release`.

if the binary isn't generated as expected, please follow the suggestion of compiler and execute `cargo fix --bin "libafl_xpdf" --allow-dirty`.

Finally, execute `./target/release/libafl_xpdf`.

The timeouts store the solutions of fuzzing the binary `pdftotext`. You can crash it by executing `./xpdf/bin/pdftotext ./timeouts/PoC`

## 0x02 Warning

I followed [this article](https://epi052.gitlab.io/notes-to-self/blog/2021-11-01-fuzzing-101-with-libafl/) and fuzzed Xpdf by LibAFL step by step. However, I fine-tuned it.

The main differences are:

- Directory structure: I removed the outermost directory. For fixing the `No such file or directory` error, I changed all relative paths to absolute paths in `main.rs`.
- The output of Monitor: It outputs a random number rather than `s`, so we can observe the execution speed of test cases.
