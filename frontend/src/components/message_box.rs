use std::hash::{Hash, Hasher};
use std::iter::once;
use color_eyre::owo_colors::OwoColorize;
use derivative::Derivative;
use ritelinked::LinkedHashSet;
use tracing::debug;
use yew::{html, AttrValue, Component, Context, Html, Properties, classes, Callback};
use crate::app::index::IndexMessage;

#[derive(Clone, Eq, Derivative, Debug)]
#[derivative(PartialEq, Hash)]
pub enum Message {
    Error(AttrValue),
    Warning(AttrValue),
    Info(AttrValue),
    More(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        i32
    ),
}

impl Message {
    fn message(&self) -> AttrValue {
        match self {
            Message::Error(m) => m.clone(),
            Message::Warning(m) => m.clone(),
            Message::Info(m) => m.clone(),
            Message::More(count) => AttrValue::from(format!("and {} more messages", count)),
        }
    }

    fn to_html(&self, rm: Callback<IndexMessage>) -> Html {
        let (class, icon) = match self {
            Message::Error(_) => (classes!("error"), "error"),
            Message::Warning(_) => (classes!("warning"), "warning"),
            Message::Info(_) => (classes!("info"), "info"),
            Message::More(_) => (classes!("more"), "add_circle")
        };

        let message = self.clone();

        let mut onclick = Some(Callback::from(move |_| {
            rm.emit(IndexMessage::RemoveMessage(message.clone()));
        }));

        if let Message::More(_) = self {
            onclick = None;
        }

        html! {
            <>
                <div { onclick } class={ classes!(class, "message") }>
                    <span class={ classes!("material-symbols-outlined") }>
                        { icon }
                    </span>
                    <span>
                        { " " }{ self.message() }
                    </span>
                </div>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct MessageBoxProps {
    pub messages: LinkedHashSet<Message>,
    pub manage_messages: Callback<IndexMessage>,
}

pub struct MessageBox;

impl Component for MessageBox {
    type Message = ();
    type Properties = MessageBoxProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let display_amount = 3;

        let mut messages = ctx
            .props()
            .messages
            .iter()
            .rev()
            // only display `display_amount` messages at a time
            .take(display_amount);

        let left = ctx.props().messages.len() as i32 - display_amount as i32;
        let msg = Message::More(left);

        let more = (left > 0).then_some(&msg);

        let messages = messages.chain(more)
            .map(|m| m.to_html(ctx.props().manage_messages.clone()))
            .collect::<Html>();

        html! {
            <>
                <div class={ classes!("messagebox-container") }>
                    { messages }
                </div>
            </>
        }
    }
}
