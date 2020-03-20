#!/bin/bash

DIR=$(cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd)

BINARY="${DIR}/target/release/tmux-session"
if [ ! -f "$BINARY" ]; then
  cd "${DIR}" && cargo build --release
fi

tmux bind-key b send-keys "RUST_LOG=debug ${BINARY} 2>${DIR}/log.txt" Enter >/dev/null 2>&1
