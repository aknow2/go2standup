#!/bin/sh

export RUSTFLAGS="--cfg=web_sys_unstable_apis"
trunk build --release --dist ../server/public
