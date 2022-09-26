#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

if ! [ -d node_modules ]; then
yarn
fi

if ! [ -x "$(command -v napi)" ]; then
npm install -g @napi-rs/cli
if [ -x "$(command -v asdf)" ]; then
asdf reshim
fi
fi

yarn build
