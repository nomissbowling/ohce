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
- cat resource/sample.txt | target/release/ohce --output resource/sample_out.txt


Options
-------

- --so-col=<RRGGBB (hex color)> : RGB color for stdout
- --se-col=<RRGGBB (hex color)> : RGB color for stderr
- -c --color : with color
- -e --error : with stderr
- --output=FILE : output to file
- -a --append : append mode (use with --output)
- -p --port=PORT : with TCP port
- --terminator=STR : close listener when received it (use with --port)


License
-------

MIT License
