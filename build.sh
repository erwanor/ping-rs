#!/bin/bash

cargo build
#setcap cap_net_raw+ep ./target/debug/ping
./target/debug/ping
