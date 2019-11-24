use essembly_core::error;
use essembly_logging::*;

//use trace::{debug, error, event, info, span, trace, warn };

#[allow(dead_code)]
pub struct Sled {}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Record {}

trait DB {
    type Format;
    fn save_record(r: Record) -> Result<(), error::Error>;
    fn create(path: &'static str, file: String) -> Result<(), error::Error>;
}

impl DB for Sled {
    type Format = Record;

    fn save_record(_r: Record) -> Result<(), error::Error> {
        Ok(())
    }

    fn create(path: &'static str, _file: String) -> Result<(), error::Error> {
        let config = sled::Config::default()
            .path(path)
            .to_owned()
            .cache_capacity(51200)
            .use_compression(false)
            .flush_every_ms(Some(100))
            .compression_factor(5)
            .snapshot_after_ops(1_000); //never

        let _db = config.open().unwrap();

        Ok(())
    }
}

#[allow(dead_code)]
pub struct Sqlite {}

//#[warn(unused_variables)]
impl DB for Sqlite {
    type Format = Record;

    fn save_record(_record: Record) -> Result<(), error::Error> {
        //info!("Saving to DB: {:?}", record);

        Ok(())
    }

    fn create(_path: &'static str, _file: String) -> Result<(), error::Error> {

        Ok(())
    }
}

// impl EssemblyDB {
//     #[allow(dead_code)]
//     pub(crate) async fn open(path: String) -> Result<File> {
//         configure_tracing();
//         let mut options = OpenOptions::new();
//         match options.create(true).read(true).write(true).open(path).await {
//             Ok(f) => Ok(f),
//             Err(e) => return Err(Error::new(ErrorCode::DBFileAccess(e.to_string()))),
//         }
//     }

//     #[allow(dead_code)]
//     pub(crate) async fn write(&self, key: String, value: String) -> Result<()> {
//         let buffer = b"Hello world!";
//         let e = tokio::fs::write("foo.txt", buffer).await;

//         match e {
//             Ok(f) => Ok(f),
//             Err(e) => return Err(Error::new(ErrorCode::DBFileAccess(e.to_string()))),
//         }
//     }
// }

// // #[instrument(level = "Debug")]
// #[allow(dead_code)]
// fn configure_tracing() {
//     let subscriber = tracing_subscriber::FmtSubscriber::builder()
//         .with_env_filter("attrs_basic=trace")
//         .finish();

//     // tracing::subscriber::with_default(subscriber, || {
//     //     tracing::debug!("Debug output from trace");
//     // });
// }
