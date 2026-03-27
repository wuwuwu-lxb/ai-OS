#!/bin/bash

# Create a pipe for serial output
PIPE=$(mktemp -u)
mkfifo $PIPE

# Start QEMU with serial output redirected to pipe
qemu-system-x86_64 -cdrom ai-os.iso -nographic -serial file:$PIPE -m 256 &
QEMU_PID=$!

# Wait for boot
sleep 2

# Read output
if [ -p $PIPE ]; then
    cat $PIPE
fi

# Cleanup
kill $QEMU_PID 2>/dev/null
rm -f $PIPE
