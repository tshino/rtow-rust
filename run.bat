@echo off
pushd %~dp0

cargo run > output.ppm && start output.ppm

popd
