Todo:

Project Overview:

- essembly directory exposes features from the essembly module
- clients always talk to an api - multiple protocols 
- apis talk to databases through secure, grpc calls 

- [x] test client for grpc
- [x] test sled persistence 
- [] abstract db into trait 
- [x] configuration read 
- [] plan Error and logging 
- [x] registration protobuf file
- [x] Separate protos to a diff project
- [x] proto-factory should generate to proto directory
- [x] protos should compile to their own crate 
- [x] separate test crate
- [] remove database refs from api 
- [] log story 
- [] intgegrate permissions 
- [] vector logging sync
- [] research wasm/wasi integration - in-place updates?


- [] Kubernetes or docker-compose:
1) Start the DB Service
2) Start the API Service
3) Run a test client ?

Todo Tests:
[x] Unit test to check missing config.toml is handled correctly fo API 

