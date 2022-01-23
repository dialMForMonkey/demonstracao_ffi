#!/bin/bash
rm /tmp/ipc.sock
node server.js &
cargo run