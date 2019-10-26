- [] test client for grpc
- [] test sled persistence 
- [] abstract db into trait 
- [] configuration read 
- [] plan Error and logging 
- [] registration protobuf file
- [] graphdb schema 


```
                                    -> Dgraph
client <-------> Servers <-------> ? 
                                    -> Sled
```

Integration Tests:

1) Can construct a Chef registration object, (protos_test),
2) Can save to DGraph with GeoLocation coordinates,
