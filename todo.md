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
- [x] api should read from generated_proto dir 
- [] research ML integration
- [] research wasm/wasi integration


Todo Tests:
1) Unit test to check missing config.toml is handled correctly


Todo Tasks:
- Build 


Accounting Systems:
1) Express accounting software (basic)
2) QuickBooks accounting (easy-most voted)
3) ACCPAC Accounting System (most voted)
4) SAP accounting software (full functions)