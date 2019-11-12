## Essembly

### Project Structure

![Project Structure](https://github.com/xenirio/essembly/blob/master/assets/essembly-project-structure.svg)

Dependencies:

         Cli
          |

       Essembly
          |

Interfaces, Core, Logging, API 


   
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


#### Startup Configuration

All crates rely on a single configuration file, `config.toml`.  The location of the startup configuration file can be changed by setting an environment variable, `ESSEMBLY_CONFIG`


There are 4 levels
trace (everything), debug(almost everything), error (big problems), , warn (little problems), info (direct messages)

#### Logging

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

