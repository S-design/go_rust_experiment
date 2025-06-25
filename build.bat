@echo off
echo building the rust library
cd rust_lib
cargo build --release
cd ..
copy rust_lib\target\release\hashlib.dll go_app\

echo done and dusted
pause