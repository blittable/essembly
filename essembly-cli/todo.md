- [x] test client for grpc
- [x] test sled persistence 
- [] abstract db into trait 
- [x] configuration read 
- [] plan Error and logging 
- [x] registration protobuf file
- [] graphdb schema 


```
                                    -> Dgraph
client <-------> Servers <-------> ? 
                                    -> Sled
```

Integration Tests:

1) Can construct a Chef registration object, (protos_test),
2) Can save to DGraph with GeoLocation coordinates,


Todo Tests:
1) Unit test to check missing config.toml is handled correctly
2) 
