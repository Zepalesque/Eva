@echo off
cd ..
set RUSTFLAGS=--emit=llvm-ir -C codegen-units=1
cargo build
if not exist .\bin\llvm mkdir .\bin\llvm
copy /Y ".\target\debug\deps\*.ll" ".\bin\llvm\"