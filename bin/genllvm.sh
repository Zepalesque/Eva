cd ..
export RUSTFLAGS="--emit=llvm-ir -C codegen-units=1"
cargo build
mkdir -p ./bin/llvm
cp ./target/debug/deps/*.ll ./bin/llvm/