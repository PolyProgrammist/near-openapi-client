#!/bin/bash
./gen.sh
cd example && cargo test -- --nocapture
