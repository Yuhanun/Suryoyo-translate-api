#!/bin/bash

cd /opt/server
sleep 10
diesel migration run
cargo run --release