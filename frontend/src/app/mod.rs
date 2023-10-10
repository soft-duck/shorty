pub mod index;

use index::Index;
use yew::{html, Component, Context, Html};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <Index/>
        }
    }
}
