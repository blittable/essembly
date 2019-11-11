use essembly::Logger;

trait Client {
    fn initialize(&self, log_level: Essembly::Logger::Level) {}
}
