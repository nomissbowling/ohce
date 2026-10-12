ohce
====

reverse echo


Links
-----

- [https://crates.io/crates/ohce](https://crates.io/crates/ohce)
- [https://github.com/nomissbowling/ohce](https://github.com/nomissbowling/ohce)


Test (both lib and bins)
------------------------

- cargo test --release -- --nocapture
- cargo test --release -- --show-output


Build (bins)
------------

- cargo build --bins --release
- cargo build --bin ohce --release
- cat resource/sample.txt | target/release/ohce


License
-------

MIT License
