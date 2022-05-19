
```
cd blinky
cargo build

# shell 1
openocd

# shell 2
gdb -x openocd.gdb -q target/thumbv7m-none-eabi/debug/blinky
```

from the gdb session:
```
c
```
admire this blinking LED :)

To flash:
```
cargo install cargo-flash
cargo flash --list-chips | grep STM32F429ZI
# both models will work
cargo flash --chip STM32F429ZITx --release
```
