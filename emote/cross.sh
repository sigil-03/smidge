#!/bin/bash
BIN_NAME=${PWD##*/}
DEPLOY_DIR=~/deploy/$BIN_NAME
cross build --target aarch64-unknown-linux-gnu
rsync ./target/aarch64-unknown-linux-gnu/debug/$BIN_NAME $1:$DEPLOY_DIR
