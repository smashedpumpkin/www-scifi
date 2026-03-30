#!/bin/sh
set -e

rm -f output/site.zip
cargo run --release
zip -j output/site.zip output/index.html
echo "Zipped → output/site.zip"
