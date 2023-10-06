# data-vis-1

Visualize some data, currently this is the [example/3dchart.rs](https://github.com/Gip-Gip/egui-plotter/blob/c13ddc7dcd2e1b5be77cb3ad92db3c256845c78e/examples/3dchart.rs).

Note: using my build of egui-plotter
```
egui-plotter = { git = "https://github.com/winksaville/egui-plotter.git", branch = "update-eframe-egui-to-0.23" }
```
so it compiles, if `egui-plotter = "0.3"` is used I see a compile `error[E0107]: struct takes 0 generic arguments but 1 generic argument was supplied`

Added src/lib.rs with my_model from which is based on
[liner-regression-explore1/example/gpt4-1.rs](https://github.com/winksaville/linear-regression-explore1/blob/1ce9cd45cdc95741b9e4962b7a9085e3d57aad21/examples/gpt4-1.rs).
I use this model to in `main()` to generate a RED square plane 1.5 x 1.5 units at a height of 1.0.

## How to run

```bash
cargo run
```

## Test

```bash
$ cargo test
   Compiling data-vis-1 v0.1.0 (/home/wink/prgs/rust/myrepos/data-vis-1)
    Finished test [unoptimized + debuginfo] target(s) in 0.52s
     Running unittests src/lib.rs (target/debug/deps/data_vis_1-f76426bdcaa1b799)

running 2 tests
test test::test1 ... ok
test test::test_36_xz_values ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/data_vis_1-1ec6672041afe936)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests data-vis-1

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Test and show output
```bash
$ cargo test -- --nocapture
   Compiling data-vis-1 v0.1.0 (/home/wink/prgs/rust/myrepos/data-vis-1)
    Finished test [unoptimized + debuginfo] target(s) in 0.51s
     Running unittests src/lib.rs (target/debug/deps/data_vis_1-f76426bdcaa1b799)

running 2 tests
parameters: [("x", 3.608224830031759e-16), ("z", -8.604228440844963e-16)]
pvalues: [("x", 0.6928347837076124), ("z", 0.6111063752059077)]
standard_errors: [("x", 6.88875270493153e-16), ("z", 1.2287917140018457e-15)]
Predicted y value for (0, 0.5): 1
inner: 36 test_data.len(): 72 x: [("x", [-0.3]), ("z", [-0.3]), ("x", [-0.3]), ("z", [-0.2]), ("x", [-0.3]), ("z", [-0.1]), ("x", [-0.3]), ("z", [0.0]), ("x", [-0.3]), ("z", [0.1]), ("x", [-0.3]), ("z", [0.2]), ("x", [-0.2]), ("z", [-0.3]), ("x", [-0.2]), ("z", [-0.2]), ("x", [-0.2]), ("z", [-0.1]), ("x", [-0.2]), ("z", [0.0]), ("x", [-0.2]), ("z", [0.1]), ("x", [-0.2]), ("z", [0.2]), ("x", [-0.1]), ("z", [-0.3]), ("x", [-0.1]), ("z", [-0.2]), ("x", [-0.1]), ("z", [-0.1]), ("x", [-0.1]), ("z", [0.0]), ("x", [-0.1]), ("z", [0.1]), ("x", [-0.1]), ("z", [0.2]), ("x", [0.0]), ("z", [-0.3]), ("x", [0.0]), ("z", [-0.2]), ("x", [0.0]), ("z", [-0.1]), ("x", [0.0]), ("z", [0.0]), ("x", [0.0]), ("z", [0.1]), ("x", [0.0]), ("z", [0.2]), ("x", [0.1]), ("z", [-0.3]), ("x", [0.1]), ("z", [-0.2]), ("x", [0.1]), ("z", [-0.1]), ("x", [0.1]), ("z", [0.0]), ("x", [0.1]), ("z", [0.1]), ("x", [0.1]), ("z", [0.2]), ("x", [0.2]), ("z", [-0.3]), ("x", [0.2]), ("z", [-0.2]), ("x", [0.2]), ("z", [-0.1]), ("x", [0.2]), ("z", [0.0]), ("x", [0.2]), ("z", [0.1]), ("x", [0.2]), ("z", [0.2])]
test test::test1 ... ok
predictions.len(): 1 predictions: [1.00] WHY is predections.len() only 1, expecting 36?
test test::test_36_xz_values ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/data_vis_1-1ec6672041afe936)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests data-vis-1

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
