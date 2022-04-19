#!/bin/sh

export RUSTFLAGS="--cfg=web_sys_unstable_apis"
#SECUARE=true
API_ORIGIN=localhost:7070 trunk serve --dist ./dist --public-url / --port 6060
