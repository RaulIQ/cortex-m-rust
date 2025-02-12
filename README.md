openocd -f interface/stlink.cfg -f target/stm32f4x.cfg 

# on a different terminal
gdb-multiarch -q target/thumbv7em-none-eabihf/debug/app
