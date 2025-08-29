# apt install protobuf-compiler 
# cargo install protobuf-codegen
protoc --rs_out ./src/protos protos/wasm_host.proto   