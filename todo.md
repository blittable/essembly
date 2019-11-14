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
- [x] remove database refs from api 
- [x] cli loads config 

- [x] log story 
- [] intgegrate permissions 
- [x] vector logging sync
- [x] research wasm/wasi integration - in-place updates?
- [x] research embedded business rules - Lua integration?
- [] research fuzzing for reverse-engineering. 
- [] Kubernetes or docker-compose
- [x] Mock test

Concrete:
- [] Logging Simple vs. Trace
- [] Real localization 
- [] Logging level based on config file reading  - https://github.com/tokio-rs/tracing/blob/bec40bc9624bcf4d26d3a5c622fdbefe4b853854/nightly-examples/examples/proxy_server.rs#L161
- [] Message to Store - format?  
- [] Store trait
- [] Extend stories 

Todo Tests:
[x] Unit test to check missing config.toml is handled correctly fo API 


