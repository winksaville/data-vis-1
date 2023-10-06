# data-vis-1

Visualize some data, currently this is the [example/3dchart.rs](https://github.com/Gip-Gip/egui-plotter/blob/c13ddc7dcd2e1b5be77cb3ad92db3c256845c78e/examples/3dchart.rs).

Note: using my build of egui-plotter
```
egui-plotter = { git = "https://github.com/winksaville/egui-plotter.git", branch = "update-eframe-egui-to-0.23" }
```
so it compiles, if `egui-plotter = "0.3"` is used I see a compile `error[E0107]: struct takes 0 generic arguments but 1 generic argument was supplied`

Added src/lib.rs with my_model from which is based on
[liner-regression-explore1/example/gpt4-1.rs](https://github.com/winksaville/linear-regression-explore1/blob/1ce9cd45cdc95741b9e4962b7a9085e3d57aad21/examples/gpt4-1.rs).
ATM the library isn't called, but it compiles and you can test it and see
the output using `cargo test test1 -- --nocapture`.

The next step is to figure out how to use `fn my_model()` in the gui.

## How to run

```bash
cargo run
```

## Test

```bash
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/data_vis_1-f76426bdcaa1b799)

running 1 test
test test::test1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/data_vis_1-1ec6672041afe936)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests data-vis-1

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Or just the one real test right now, `test1`
```bash
$ cargo test test1 -- --nocapture
   Compiling data-vis-1 v0.1.0 (/home/wink/prgs/rust/myrepos/data-vis-1)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests src/lib.rs (target/debug/deps/data_vis_1-f76426bdcaa1b799)

running 1 test
Predicted y value for (0, 0.5): 1
test test::test1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/data_vis_1-1ec6672041afe936)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.