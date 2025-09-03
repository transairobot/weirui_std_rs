# apt install protobuf-compiler 
# cargo install protobuf-codegen
protoc --rs_out ./src/host/pb ./src/host/pb/host_pb.proto
# pb-rs src/host/host_pb.proto