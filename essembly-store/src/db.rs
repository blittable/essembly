use crate::susu_error::{Error, ErrorCode, Result};

use tokio::fs::{File, OpenOptions};

use std::collections::HashMap;
use std::path::PathBuf;

use futures::{Future, Stream, future, stream};
#[cfg(feature = "tokio")]
use std::rc::Rc;
use tokio;
#[cfg(feature = "tokio")]
use tokio::runtime;
use tokio::runtime::current_thread;
use futures::lock::Mutex;

use tracing::{debug, instrument, log, Subscriber};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tracing_attributes;
use tracing_futures;

pub struct SusuDB {
    hashmap: Mutex<HashMap<String, String>>
}

impl SusuDB {

    pub(crate) async fn open(path: String) -> Result<File> {
        configure_tracing();
        let mut options = OpenOptions::new();
        match options.create(true).read(true).write(true).open(path).await {
            Ok(f) => Ok(f),
            Err(e) => return Err(Error::new(ErrorCode::DBFileAccess(e.to_string()))),
        }
    }
}

#[instrument(level = "Debug")]
fn configure_tracing() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter("attrs_basic=trace")
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        tracing::debug!("Debug output from trace");
    });
}
