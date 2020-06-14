@echo off
pushd %~dp0

cargo run --release > output.ppm && start output.ppm

popd
