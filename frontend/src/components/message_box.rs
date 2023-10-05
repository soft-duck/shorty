use linked_hash_set::LinkedHashSet;
use yew::{html, AttrValue, Component, Context, Html, Properties};

#[derive(PartialEq, Clone, Hash, Eq)]
pub enum Message {
    Error(AttrValue),
    Warning(AttrValue),
    Info(AttrValue),
}

impl Message {
    fn message(&self) -> &AttrValue {
        match self {
            Message::Error(m) => m,
            Message::Warning(m) => m,
            Message::Info(m) => m,
        }
    }

    fn to_html(&self) -> Html {
        // TODO make more concise
        let message_type = match self {
            Message::Error(m) => "Error: ",
            Message::Warning(m) => "Warning: ",
            Message::Info(m) => "Info: ",
        };

        html! {
            <>
                <div>
                    <span>{ message_type } { self.message() }</span>
                </div>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct MessageBoxProps {
    pub messages: LinkedHashSet<Message>,
}

pub struct MessageBox;

impl Component for MessageBox {
    type Message = ();
    type Properties = MessageBoxProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let messages = ctx
            .props()
            .messages
            .iter()
            .map(|m| m.to_html())
            .collect::<Html>();

        html! {
            <>
                <div>
                    { messages }
                </div>
            </>
        }
    }
}
