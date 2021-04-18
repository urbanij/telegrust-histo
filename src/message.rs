use chrono::prelude::DateTime;
use chrono::Utc;

// #[derive(Debug)]
pub struct Message {
    pub timestamp: f64,
}

impl Message {
    pub fn new(ts: f64) -> Self {
        Self {
            timestamp: ts,
        }
    }

    pub fn get_timestamp_string(&self) -> String {
        self.convert_epoch_to_readable_date()
    }
    
    fn convert_epoch_to_readable_date(&self) -> String {
        // Creates a new SystemTime from the specified number of whole seconds
        let d = std::time::UNIX_EPOCH + std::time::Duration::from_secs(self.timestamp as u64);
        // Create DateTime from SystemTime
        let datetime = DateTime::<Utc>::from(d);
        // Formats the combined date and time with the specified format string.
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M").to_string();

        timestamp_str
    }
}

