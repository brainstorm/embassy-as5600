# Quickstart

Flash the Arduino Mega 2560 board with, YMMV:

```sh
cargo run -- -P /dev/ttyUSB0                # Linux
cargo run -- -P /dev/cu.usbserial-124430    # OSX
```

# Prerequisites

```
$ apt-get install avrdude clang pkg-config gcc-avr gdb-avr libelf-dev # Linux
$ brew install avrdude                                                # OSX
$ cargo install ravedude
```

WARNING: Currently crashes on real hardware, possible illegal instruction or more obscure error that needs [close debugging attention](https://n-eq.github.io/blog/2025/05/13/rust-avr-arduino-blink).
