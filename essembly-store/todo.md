- [] test client for grpc
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
