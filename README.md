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