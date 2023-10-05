mod index;

use index::Index;
use yew::{html, Component, Context, Html};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Index/>
        }
    }
}
