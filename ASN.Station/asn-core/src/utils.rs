#[macro_export]
macro_rules! asn_log {
    ($($arg:tt)*) => {

        use std::sync::OnceLock;

        static STATION_ID: OnceLock<String> = OnceLock::new();
        let station_id = STATION_ID.get_or_init(|| {
            if cfg!(debug_assertions) {
                std::env::var("STATION_ID").unwrap_or_else(|_| "ASN-DEV".to_string())
            } else {
                env!("STATION_ID").into()
            }
        });

        println!("[{}]: {}", station_id, format!($($arg)*));
    };
}

#[macro_export]
macro_rules! asn_err {
    ($($arg:tt)*) => (
        eprintln!("\x1b[31m[ERROR] [{}]: {}\x1b[0m", env!("STATION_ID"), format!($($arg)*))
    );
}

#[macro_export]
macro_rules! asn_debug {
    ($($arg:tt)*) => (
        if cfg!(debug_assertions) {
            println!("\x1b[36m[DEBUG] [{}]: {}\x1b[0m", env!("STATION_ID"), format!($($arg)*))
        }
    );
}

#[macro_export]
macro_rules! asn_info {
    ($($arg:tt)*) => (
        println!("\x1b[32m[INFO] [{}]: {}\x1b[0m", env!("STATION_ID"), format!($($arg)*))
    );
}
