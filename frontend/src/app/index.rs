use gloo_timers::callback::Timeout;
use linked_hash_set::LinkedHashSet;
use yew::{html, Component, Context, Html, classes, AttrValue};

use crate::components::{
    link_form::LinkForm,
    message_box::{Message, MessageBox},
    footer::Footer,
};

pub enum IndexMessage {
    AddMessage(Message),
    RemoveMessage(Message),
}

pub struct Index {
    messages: LinkedHashSet<Message>,
}

impl Component for Index {
    type Message = IndexMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let mut messages = LinkedHashSet::new();
        // let message = AttrValue::from("A looooooooooooong message to see css styling effects");
        //
        // messages.insert(Message::Error(message.clone()));
        // messages.insert(Message::Warning(message.clone()));
        // messages.insert(Message::Info(message));

        Self {
            messages,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexMessage::AddMessage(m) => {
                self.messages.insert(m.clone());

                let callback = ctx.link().callback(|m| IndexMessage::RemoveMessage(m));
                let timeout = Timeout::new(5_000, move || {
                    callback.emit(m);
                });

                timeout.forget();
            },
            IndexMessage::RemoveMessage(m) => {
                self.messages.remove(&m);
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_message = ctx.link().callback(|m| IndexMessage::AddMessage(m));
        let remove_message = ctx.link().callback(|m| IndexMessage::RemoveMessage(m));

        html! {
            <>
                <div class={ classes!("page-container") }>
                    // TODO clone could be mitigated with an appropriate pointer type
                    <MessageBox { remove_message } messages={ self.messages.clone() }/>
                    <div class={ classes!("link-shortener-group-container") }>
                        <div class={ classes!("link-shortener-group") }>
                            <LinkForm callback={ add_message }/>
                        </div>
                    </div>
                    <Footer/>
                </div>
            </>
        }
    }
}
