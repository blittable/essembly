use super::error;

#[allow(dead_code)]
struct Sled {}

#[allow(dead_code)]
struct Record {}

trait DB {
    type Format;
    fn save_record(r: Record) -> Result<(), error::Error>;
}

#[warn(unused_variables)]
impl DB for Sled {
    type Format = Record;

    fn save_record(_r: Record) -> Result<(), error::Error> {
        Ok(())
    }
}

// impl SusuDB {
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
