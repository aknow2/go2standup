#!/bin/sh

export RUSTFLAGS="--cfg=web_sys_unstable_apis"
trunk serve --dist ./dist --public-url / --port 6060
