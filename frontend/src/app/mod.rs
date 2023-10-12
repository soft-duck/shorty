use index::Index;
use stylist::{css, yew::Global, StyleSource};
use yew::{html, Component, Context, Html};

use crate::BACKGROUND_COLOR;

pub mod index;

pub struct App;

thread_local! {
    static GLOBAL: StyleSource = css!(r#"
        body {
            margin: 0;
            background-color: ${bg};
        }
    "#, bg = BACKGROUND_COLOR);
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                <Global css={ GLOBAL.with(|s| s.clone()) } />
                <Index/>
            </>
        }
    }
}
