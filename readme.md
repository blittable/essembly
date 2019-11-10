## Essembly

### Project Structure

![Project Structure](https://github.com/xenirio/essembly/blob/master/assets/essembly-project-structure.svg)

### Testing

Most code is tested in the .rs files themselves.  The generated code (from the protobufs) is tested in the essembly-test project (not a release assembly).  

Integration tests are in the `essembly-test` project.  

#### Running Unit Tests

From the root directory, `cargo test --all`

Note: This does not run the integration tests in the essembly-test crate

#### Running Essembly Integration Tests
From the essembly-test directory, `cargo test`

### Development Testing

The reference client is the essembly-cli:   `cargo run --bin essembly-cli --help` 
The cli, and other crates, references the one (and only config file).  The supporting crate is essembly-config 

Starting it does the following:

(e.g. `cargo run --bin essembly-cli acct post {"x", "y"} 

1) Reads the .config file at the root of the project (there is 1 .config file for the entire project)
2) Used the libraries in core (e.g. account) to validate the data,
3) Connects to the API,
4) Uses the API to commit to database  
