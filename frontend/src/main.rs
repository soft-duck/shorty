use gloo_timers::callback::Timeout;
use linked_hash_set::LinkedHashSet;
use tracing::Level;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use yew::prelude::*;

use link_form::LinkForm;
use message_box::MessageBox;

use crate::message_box::Message;
use crate::util::fetch_server_config;

mod link_input;
mod advanced_mode;
mod toggle;
mod expiration_mode;
mod util;
mod duration_input;
mod link_form;
mod message_box;
mod link_config;


enum AppMessage {
    AddMessage(Message),
    ClearMessage(Message),
}

struct App {
    messages: LinkedHashSet<Message>,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            messages: LinkedHashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::AddMessage(m) => {
                self.messages.insert(m.clone());

                let callback = ctx.link().callback(|m| AppMessage::ClearMessage(m));
                let timeout = Timeout::new(5_000, move || {
                    callback.emit(m);
                });

                timeout.forget();
            },
            AppMessage::ClearMessage(m) => { self.messages.remove(&m); },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(|m| {
            AppMessage::AddMessage(m)
        });

        html! {
            <>
                <MessageBox messages={ self.messages.clone() }/>
                <h1>{ "[WIP] Link Shortener" }</h1>
                <LinkForm callback={ callback }/>
            </>
        }
    }
}

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
        - build or get a good duration input
        - css styling of the page
            - this includes redoing the message format or rephrasing error messages
        - migrate to yew 0.21.0
    TODO less important:
        - checkout if the form name coupling can be made more concise
            - maybe use contexts
            - or by using node_refs and dropping the name entirely (currently sounds the best)
        - decide between console_error_panic_hook and color_eyre and decide if any of these is needed at all
        - add a footer to the page for stuff like "about", "github source", ...
        - organize the components in different modules and maybe spilt or merge a few files
            - maybe after atomic design logic, but that would need a bigger restructure
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
