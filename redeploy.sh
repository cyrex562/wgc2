#!/bin/bash
git pull origin main
cargo build
sudo ./target/debug/wgc2 -a 10.255.0.7 -p 8999