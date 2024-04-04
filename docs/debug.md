# Debugging

We'll be using `simavr`+`avr-gdb` because [debugWire is impractically hard to setup](https://github.com/Rahix/avr-hal/discussions/399#discussioncomment-9008395).

On Ubuntu, to avoid having GDB-Python errors, make sure the following `gdbevents` line in `/usr/share/gdb/python/gdb/` is commented out:

```python
# Historically, gdb.events was always available, so ensure it's
# still available without an explicit import.
#import _gdbevents as events
```

Now, open two terminals, on the `simavr` terminal you'll see the following on a full debugging session:

```
$ simavr --mcu atmega2560 target/avr-atmega2560/debug/embassy-avr-as5600-encoder.elf -g
Loaded 9950 .text at address 0x0
Loaded 3146 .data
avr_gdb_init listening on port 1234
gdb_network_handler connection opened
Reading angle.
Firmware panic!..
  At src/main.rs:63:38..
GDB hit control-c
```

And on the GDB terminal:

```
$ avr-gdb
(gdb) file target/avr-atmega2560/debug/embassy-avr-as5600-encoder.elf
Reading symbols from target/avr-atmega2560/debug/embassy-avr-as5600-encoder.elf...
(gdb) target remote :1234
Remote debugging using :1234
avr_device::interrupt::restore (irq_flag=...) at src/interrupt.rs:170
170	            asm!(
(gdb) c
Continuing.
^C
Program received signal SIGTRAP, Trace/breakpoint trap.
0x00000910 in avr_hal_generic::delay::busy_loop (c=16375) at /home/rvalls/.cargo/git/checkouts/avr-hal-f3855d5807fdfd57/2eb28fa/avr-hal-generic/src/delay.rs:52
52	                asm!(
(gdb) bt
#0  0x00000910 in avr_hal_generic::delay::busy_loop (c=16375)
    at /home/rvalls/.cargo/git/checkouts/avr-hal-f3855d5807fdfd57/2eb28fa/avr-hal-generic/src/delay.rs:52
#1  avr_hal_generic::delay::{impl#3}::delay_us (us=16375, self=<optimized out>)
    at /home/rvalls/.cargo/git/checkouts/avr-hal-f3855d5807fdfd57/2eb28fa/avr-hal-generic/src/delay.rs:157
#2  avr_hal_generic::delay::{impl#9}::delay_us<avr_hal_generic::clock::MHz16> (us=100000, self=<optimized out>)
    at /home/rvalls/.cargo/git/checkouts/avr-hal-f3855d5807fdfd57/2eb28fa/avr-hal-generic/src/delay.rs:273
#3  avr_hal_generic::delay::{impl#10}::delay_ms<avr_hal_generic::clock::MHz16> (ms=100, self=<optimized out>)
    at /home/rvalls/.cargo/git/checkouts/avr-hal-f3855d5807fdfd57/2eb28fa/avr-hal-generic/src/delay.rs:285
#4  arduino_hal::delay::delay_ms (ms=100)
    at /home/rvalls/.cargo/git/checkouts/avr-hal-f3855d5807fdfd57/2eb28fa/arduino-hal/src/delay.rs:14
#5  embassy_avr_as5600_encoder::panic (info=<optimized out>) at src/main.rs:43
#6  0x00001c6c in core::panicking::panic_fmt (fmt=...) at src/panicking.rs:72
#7  0x00001d7c in core::result::unwrap_failed (msg=..., error=...) at src/result.rs:1654
#8  0x000006f2 in core::result::Result<u16, as5600::error::Error<avr_hal_generic::i2c::Error>>::unwrap<u16, as5600::error::Error<avr_hal_generic::i2c::Error>> (self=...)
    at /home/rvalls/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1077
#9  embassy_avr_as5600_encoder::____embassy_main_task::{async_fn#0} () at src/main.rs:63
#10 embassy_executor::raw::TaskStorage<embassy_avr_as5600_encoder::____embassy_main_task::{async_fn_env#0}>::poll<embassy_avr_as5600_encoder::____embassy_main_task::{async_fn_env#0}> (p=...)
    at /home/rvalls/.cargo/git/checkouts/embassy-9312dcb0ed774b29/1171e11/embassy-executor/src/raw/mod.rs:159
#11 0x000025c6 in embassy_executor::raw::{impl#9}::poll::{closure#0} (p=...)
    at /home/rvalls/.cargo/git/checkouts/embassy-9312dcb0ed774b29/1171e11/embassy-executor/src/raw/mod.rs:405
#12 embassy_executor::raw::run_queue::RunQueue::dequeue_all<embassy_executor::raw::{impl#9}::poll::{closure_env#0}> (
    self=<optimized out>, on_task=...)
    at /home/rvalls/.cargo/git/checkouts/embassy-9312dcb0ed774b29/1171e11/embassy-executor/src/raw/run_queue_critical_section.rs:72
#13 embassy_executor::raw::SyncExecutor::poll (self=<optimized out>)
    at /home/rvalls/.cargo/git/checkouts/embassy-9312dcb0ed774b29/1171e11/embassy-executor/src/raw/mod.rs:386
#14 embassy_executor::raw::Executor::poll (self=<optimized out>)
    at /home/rvalls/.cargo/git/checkouts/embassy-9312dcb0ed774b29/1171e11/embassy-executor/src/raw/mod.rs:526
#15 embassy_executor::arch::thread::Executor::run<embassy_avr_as5600_encoder::__avr_device_rt_main::{closure_env#0}> (
    self=<optimized out>, init=...)
    at /home/rvalls/.cargo/git/checkouts/embassy-9312dcb0ed774b29/1171e11/embassy-executor/src/arch/avr.rs:66
#16 0x00000962 in embassy_avr_as5600_encoder::__avr_device_rt_main () at src/main.rs:47
#17 0x0000093a in embassy_avr_as5600_encoder::__avr_device_rt_main_trampoline () at src/main.rs:47
(gdb)
```
