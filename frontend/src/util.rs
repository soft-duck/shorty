use std::{
    ops::Deref,
    sync::{Mutex, OnceLock, RwLock},
    thread::LocalKey,
};

use time::UtcOffset;
use tiny_id::{ExhaustionStrategy, ShortCodeGenerator};
use tracing::{debug, warn, Level};
use tracing_subscriber::{
    fmt::{time::UtcTime, writer::MakeWriterExt},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use yew::{classes, platform::spawn_local, AttrValue, Classes};

use crate::types::ServerConfig;

static SHORT_CODE_GENERATOR: OnceLock<Mutex<ShortCodeGenerator<char>>> = OnceLock::new();
static SERVER_CONFIG: OnceLock<RwLock<ServerConfig>> = OnceLock::new();
static ORIGIN: OnceLock<RwLock<String>> = OnceLock::new();

/// generate a guaranteed to be unique alphanumeric id
pub fn generate_id() -> AttrValue {
    AttrValue::from(
        SHORT_CODE_GENERATOR
            .get_or_init(|| {
                let generator = ShortCodeGenerator::new_alphanumeric(4)
                    .exhaustion_strategy(ExhaustionStrategy::IncreaseLength);

                Mutex::new(generator)
            })
            .lock()
            .unwrap()
            .next_string(),
    )
}

pub fn origin() -> impl Deref<Target = String> {
    // FIXME handle unwrap
    ORIGIN
        .get_or_init(|| RwLock::new(web_sys::window().unwrap().origin()))
        .read()
        .unwrap()
}

#[macro_export]
macro_rules! endpoint {
    ($($arg:tt)*) => {{
        format!("{}/{}", *crate::util::origin(), format_args!($($arg)*))
    }}
}

pub fn fetch_server_config() {
    spawn_local(async {
        debug!("Fetching server config...");
        let result = reqwest::get(endpoint!("config")).await;

        match result {
            Ok(response) => {
                if let Ok(config) = response.json::<ServerConfig>().await {
                    debug!("Successfully fetched config: {:#?}", config);
                    SERVER_CONFIG.set(RwLock::new(config)).unwrap();
                }
            },
            Err(e) => warn!("Fetching server config failed with: {}", e),
        }
    });
}

pub fn try_get_local_offset() -> Option<UtcOffset> {
    match UtcOffset::current_local_offset() {
        Ok(offset) => Some(offset),
        Err(e) => {
            debug!("Unable to get local offset: {}", e);
            None
        },
    }
}

pub fn server_config() -> Option<impl Deref<Target = ServerConfig>> {
    SERVER_CONFIG.get().map(|c| c.read().unwrap())
}

pub fn setup_tracing_subscriber() {
    // done with consts because of https://github.com/rust-lang/rust/issues/15701
    #[cfg(debug_assertions)]
    const LEVEL: Level = Level::DEBUG;

    #[cfg(not(debug_assertions))]
    const LEVEL: Level = Level::WARN;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(tracing_web::MakeConsoleWriter.with_max_level(LEVEL));

    tracing_subscriber::registry().with(fmt_layer).init();
}

pub trait AsClasses {
    fn as_classes(&'static self) -> Classes;
}

impl<T: Clone> AsClasses for LocalKey<T>
where
    Classes: From<T>,
{
    fn as_classes(&'static self) -> Classes {
        // should be fine because T is a StyleSource and StyleSource only holds Rc's
        self.with(|a| classes!(a.clone()))
    }
}
