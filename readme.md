## Essembly

### Project Structure

### Testing

Most code is tested in the .rs files themselves.  The generated code (from the protobufs) is tested in the essembly-test project (not a release assembly).  

Other integration tests are in the `essembly-test` project.  

#### Running Unit Tests
From the root directory, `cargo test`

Note: This does not run the integration tests in the essembly-test crate

#### Running Essembly Integration Tests
From the essembly-test directory, `cargo test`


