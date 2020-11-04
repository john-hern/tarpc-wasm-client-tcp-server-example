use wasm_bindgen::JsValue;
use web_sys::console::log_1;

//TODO: Implement a buffer if needed.
pub struct LogWriter {
    level: log::Level,
}
impl LogWriter {
    pub fn new(level: log::Level) -> Self {
        Self { level }
    }
}

impl log::Log for LogWriter {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            let current_thread = std::thread::current();

            let thread_id = current_thread.id();
            let thread_name = current_thread.name().or_else(|| Some("")).unwrap();
            let msg = format!(
                "{} - {:?}:{} - {}:{:#?} -- {}",
                record.level(),
                thread_id,
                thread_name,
                record.target(),
                record.line().unwrap_or_default(),
                record.args()
            );
            log_1(&JsValue::from_str(&msg));
        }
    }

    fn flush(&self) {}
}
