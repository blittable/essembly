use tokio::fs::{File, OpenOptions};
use std::path::PathBuf;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use failure::{bail, Error, Fallible};

#[cfg(feature = "tokio")]
use std::rc::Rc;
use tokio;

pub struct ImportOp {}

impl ImportOp {
    pub(crate) async fn open(path: String) -> Result<(), std::io::Error> {
        let mut options = OpenOptions::new();
        let f = options.create(true).read(true).write(true).open(path).await?;

         Ok(())
    }
}


