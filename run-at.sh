#!/usr/bin/env sh
DIR="$(dirname "$(realpath "$0")")"

set -e

cargo build

cd ~/Agroop/plataform/frontend

$DIR/target/debug/nyp ni