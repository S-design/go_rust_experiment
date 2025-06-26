@echo off
echo building the rust library
cd rust_lib
cargo build --release
cd ..
echo copying dll to the go app folder
copy rust_lib\target\release\hashlib.dll hash_cli\
echo done and dusted
pause