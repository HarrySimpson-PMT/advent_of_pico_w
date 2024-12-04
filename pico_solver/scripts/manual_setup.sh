#this code is for creating a file that can be transfered to the pico using the boot button over usb. working as of 2024/12/04

#!/bin/bash
set -e  # Stop on first error

BINARY_NAME="wifi_tcp_server"

# Build the binary - this might not work since its in a workspace now, depends on where you run it from.
echo "Building $BINARY_NAME..."
cargo build --release --bin $BINARY_NAME

# Convert to UF2
echo "Converting to UF2..."
elf2uf2-rs target/thumbv6m-none-eabi/release/$BINARY_NAME

# Provide feedback
echo "UF2 file created: target/thumbv6m-none-eabi/release/$BINARY_NAME.uf2"

# Optional: Flash to Pico
# echo "Flashing the Pico..."
# cp target/thumbv6m-none-eabi/release/$BINARY_NAME.uf2 /media/$USER/RPI-RP2