## The proto-factory project generates code using the files in the proto directory.  The output files are put in the /generated_protos directory at the root of the essembly project.  Two environment variables can also be used to configure the source and target directories.

```PROTO_INPUT_DIR```
```PROTO_OUTPUT_DIR```

## The proto-factory project is defined with [workspace], meaning that it is not included in release code.

## Building
The project is built with `cargo run` and executes the main.rs