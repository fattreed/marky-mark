#!/bin/sh

cargo clippy -- -D clippy::pedantic -D clippy::nursery -D clippy::unwrap_used -D clippy::expect_used
