use crate::app::App;
use crate::util::{fetch_server_config, setup_tracing_subscriber};

mod util;
mod components;
mod app;
mod types;


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
        - rework build process (this includes changing the dockerfile)
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
