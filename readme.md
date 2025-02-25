## Essembly: Developer Notes

### Project Structure

`essembly-cli` is a reference client that uses all functionality.  Other clients (e.g. mobile, desktop) can be built using the cli's code.

Details on the other crates can be found by viewing the readme documents at the root of each crate:

[`essembly`](essembly/readme.md)

[`essembly-cli`](essembly-cli/readme.md)

[`essembly-config`](essembly-config/readme.md)

[`essembly-core`](essembly-core/readme.md)

[`essembly-interfaces`](essembly-interfaces/readme.md)

[`essembly-logging`](essembly-logging/readme.md)

[`essembly-store`](essembly-store/readme.md)

[`essembly-test`](essembly-test/readme.md)



Runnable Crate Dependencies (business logic is contained in `essembly-core`):

```
                       cli                            
                        |                             
               Essembly (parent crate)                
                        |
         Interfaces, Config, Logging, Core
              |
             Core

                       api 
                        |
         Interfaces, Config, Logging, Core
              |
             Core

                      store 
                        |
         Interfaces, Config, Logging, Core?
              |
             Core

```

   
### Testing

Most code is tested in the .rs files themselves.  

The generated code (from the protobufs) is tested in the `essembly-test` project (not a release assembly).  

Integration tests are in the `essembly-test` project.  

#### Running Unit Tests

From the root directory, `cargo test --all`

Note: This does not run the integration tests in the essembly-test crate

#### Running Essembly Integration Tests (Manual)

First, run `cargo run essembly-api` to start the API server
From the essembly-test directory, `cargo test`


#### Run the Essembly client from the cli 

First, run `cargo run essembly-api` to start the API server

The reference client is the essembly-cli:   `cargo run --bin essembly-cli --help` 

The cli, and other crates, references the one (and only config file).  The supporting crate is essembly-config 

Starting it does the following:

(e.g. `cargo run --bin essembly-cli acct post {"x", "y"} 

1) Reads the .config file at the root of the project (there is 1 .config file for the entire project)
2) Used the libraries in core (e.g. account) to validate the data,
3) Connects to the API,
4) Uses the API to commit to database  

Note - the essembly-cli does not do direct to db, but requires the API.

### Configuration

All crates rely on a single configuration file, `config.toml`.  The location of the startup configuration file can be changed by setting an environment variable, `ESSEMBLY_CONFIG`

### Logging

There are 5 logging levels:

1. trace (everything), 
2. debug(almost everything), 
3. error (big problems), 
4. warn (little problems), 
5. info (direct messages)

Currently, the logging is using the tokio-trace library (`subscriber` below).  There is also a toy simple logger that does *not* yet implement the std::log trait.

(essembly-api sample)

```rust
 let config = Config::new().load();
 println!("API configuration: {:?}", config.api);
 println!("API logging configuration: {:?}", config.logger);

 let logger = &mut essembly::logging::simple::SimpleLogger::new();
 logger.initialize(essembly::logging::Level::DEBUG);
 logger.log(essembly::logging::Level::DEBUG, "foo".to_string());

 let subscriber = tracing_subscriber::fmt::Subscriber::builder()
   .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

 info!("server started");
```

##

Planning:



Blackbox Testing

1.    


