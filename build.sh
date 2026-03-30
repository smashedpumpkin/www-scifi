#!/bin/sh
set -e

rm -f output/site.zip
cargo run --release
zip -j output/site.zip output/index.html content/icon-x-small.png

echo ""
echo "✓ Build complete"
echo ""
zipinfo output/site.zip
