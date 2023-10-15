use crate::{app::App, util::setup_tracing_subscriber};

mod app;
mod components;
mod types;
mod util;

pub const BACKGROUND_COLOR: &str = "#1C1C1C";
pub const ACCENT_COLOR: &str = "#DC143C";
pub const FONT_COLOR: &str = "white";
pub const FONT_FAMILY: &str = "'Roboto Slab', serif";
pub const INPUT_WIDTH: &str = "235px";

/*
    TODO:
        - add server config params to html elements
            - needs a callback to update the element when server config is fetched
        - css styling of the page
            - fix layout shift
            - mobile friendly
            - adjust pixel distances
            - this includes redoing the message format or rephrasing error messages
        - look into implicit_clone instead of AsClasses or storing static Classes
    TODO less important:
        - fix firefox number input
        - improve link_config parsing code
            - this includes better errors with input.validity
        - look into caching
            - fonts / assets
            - webpage
        - should callbacks always be assigned in the view() method or is there another way (static?), that's more optimized
        - do not use the App component as as a middleman but talk directly to message box
        - write test-cases for ui (how?)
        - rerender on server config
    TODO @flamion:
        - negative max_uses, valid_for
        - backend error enum or similar to give more specific messages
            - alternately display the backend error directly
        - rework build process (this includes changing the dockerfile)
            - decision needed
*/
fn main() {
    setup_tracing_subscriber();

    yew::Renderer::<App>::new().render();
}
