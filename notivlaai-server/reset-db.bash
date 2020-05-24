#!/usr/bin/env bash
diesel migration redo
cargo r --bin seed
