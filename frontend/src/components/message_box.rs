use color_eyre::owo_colors::OwoColorize;
use linked_hash_set::LinkedHashSet;
use yew::{html, AttrValue, Component, Context, Html, Properties, classes, Callback};

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

    fn to_html(&self, rm: Callback<Message>) -> Html {
        let (class, icon) = match self {
            Message::Error(_) => (classes!("error"), "error"),
            Message::Warning(_) => (classes!("warning"), "warning"),
            Message::Info(_) => (classes!("info"), "info"),
        };

        let message = self.clone();

        let onclick = Callback::from(move |_| {
            rm.emit(message.clone());
        });

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
    pub remove_message: Callback<Message>,
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
            .rev()
            .map(|m| m.to_html(ctx.props().remove_message.clone()))
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
