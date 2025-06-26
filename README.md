# Rust-Go SHA256 Hasher

A project displaying how to make a high performance SHA-256 hashing function with Rust, calling from a Go app using FFI (cgo).

## Structure
- `rust_lib/` — A Rust library containing SHA-256 logic
- `go_app/` — A Go app that calling the Rust function
- `build.sh` — This builds the Rust library

## How to Run

```powershell
./build.sh
cd go_app
go run main.go
