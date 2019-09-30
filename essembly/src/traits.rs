pub trait SysTrx {
    type id;
    fn record();
    fn log();
}