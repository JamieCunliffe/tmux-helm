#!/bin/bash

DIR=$(cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd)
BINARY="${DIR}/target/release/tmux-session"
RUST_LOG=info ${BINARY} 2>>${DIR}/log.txt
