use gloo_timers::callback::Timeout;
use linked_hash_set::LinkedHashSet;
use yew::{html, Component, Context, Html};

use crate::components::{
    link_form::LinkForm,
    message_box::{Message, MessageBox},
};

pub enum IndexMessage {
    AddMessage(Message),
    ClearMessage(Message),
}

pub struct Index {
    messages: LinkedHashSet<Message>,
}

impl Component for Index {
    type Message = IndexMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            messages: LinkedHashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexMessage::AddMessage(m) => {
                self.messages.insert(m.clone());

                let callback = ctx.link().callback(|m| IndexMessage::ClearMessage(m));
                let timeout = Timeout::new(5_000, move || {
                    callback.emit(m);
                });

                timeout.forget();
            },
            IndexMessage::ClearMessage(m) => {
                self.messages.remove(&m);
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(|m| IndexMessage::AddMessage(m));

        html! {
            <>
                <MessageBox messages={ self.messages.clone() }/>
                <h1>{ "[WIP] Link Shortener" }</h1>
                <LinkForm callback={ callback }/>
            </>
        }
    }
}
