#!/bin/sh

DEPLOY=deploy

rm -rf $DEPLOY
mkdir -p $DEPLOY
cargo build --release
cp target/release/dalia-challenge $DEPLOY
cp LICENSE $DEPLOY
cp README.md $DEPLOY
cp -r images $DEPLOY
zip -r $DEPLOY $DEPLOY