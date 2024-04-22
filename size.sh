#!/bin/bash
cargo b --release 2> /dev/null && strip target/release/stab && exa -lB target/release/stab | awk '{ print $2 }'
