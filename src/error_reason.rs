use std::sync::Mutex;

pub struct ErrorReason {
    error_provider: Mutex<Option<Box<dyn 'static + Sync + Send + FnOnce() -> String>>>,
    reason_str: std::sync::OnceLock<String>,
}

impl std::fmt::Debug for ErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorReason({:?})", self.as_str())
    }
}

impl std::fmt::Display for ErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ErrorReason {
    pub fn new(producer: impl 'static + Sync + Send + FnOnce() -> String) -> Self {
        Self {
            error_provider: Mutex::new(Some(Box::new(producer))),
            reason_str: std::sync::OnceLock::new(),
        }
    }

    pub fn as_str(&self) -> &str {
        let result = self
            .reason_str
            .get_or_init(|| match self.error_provider.lock() {
                Err(e) => {
                    panic!(
                        "typed-error internal error: cannot lock to get error provider: {}",
                        e
                    );
                }
                Ok(mut error_producer) => {
                    let error_producer = error_producer.take();
                    match error_producer {
                        None => panic!("typed-error internal error: provider has been consumed"),
                        Some(error_producer) => (error_producer)(),
                    }
                }
            });

        result.as_str()
    }
}
