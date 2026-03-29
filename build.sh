#!/bin/sh
set -e

cargo run --release
rm -f output/site.zip
zip -j output/site.zip output/index.html
echo "Zipped → output/site.zip"
