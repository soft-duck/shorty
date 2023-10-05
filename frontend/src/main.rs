use tracing::Level;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use yew::prelude::*;

use crate::app::App;
use crate::util::fetch_server_config;

mod util;
mod components;
mod app;
mod types;

fn setup_tracing_subscriber() {
    // done with consts because of https://github.com/rust-lang/rust/issues/15701
    #[cfg(debug_assertions)]
    const LEVEL: Level = Level::DEBUG;

    #[cfg(not(debug_assertions))]
    const LEVEL: Level = Level::WARN;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(tracing_web::MakeConsoleWriter.with_max_level(LEVEL));

    tracing_subscriber::registry()
        .with(fmt_layer)
        .init();
}


/*
    TODO:
        - create error types for error messages
        - create the parsing logic
        - incorporate the errors into the parsing logic
        - add validation
            - to the frontend ui
            - before sending a request
        - add server config params to html elements
        - css styling of the page
            - this includes redoing the message format or rephrasing error messages
        - migrate to yew 0.21.0
    TODO less important:
        - decide between console_error_panic_hook and color_eyre and decide if any of these is needed at all
        - add a footer to the page for stuff like "about", "github source", ...
        - should callbacks always be assigned in the view() method or is there another way (static?), that's more optimized
        - do not use the App component as as a middleman but talk directly to message box
        - currently the logic for "disabling" the submit functionality just change the button type.
          It wold be cleaner if the submit event would be removed entirely
        - write test-cases for ui (how?)
        - check what the difference between props with ~ and without ~ is
            - answer: https://discord.com/channels/701068342760570933/703449306497024049/1158206184063508501
            - so in our case we always want to use properties without ~
*/
fn main() {
    setup_tracing_subscriber();
    // used to fetch the server config in the beginning
    fetch_server_config();

    // panic::set_hook(Box::new(console_error_panic_hook::hook));

    yew::Renderer::<App>::new().render();
}
