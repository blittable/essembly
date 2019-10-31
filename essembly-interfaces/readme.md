## The essembly-interfaces crate generates code using the files in the proto directory.  The output files are put in the /src directory .  Two environment variables can also be used to configure the source and target directories. 

```PROTO_INPUT_DIR```
```PROTO_OUTPUT_DIR```

## Building
The project is built with `cargo build` and executes the `tonic-build`