use derivative::Derivative;
use ritelinked::LinkedHashSet;
use stylist::{css, StyleSource};
use yew::{classes, html, AttrValue, Callback, Component, Context, Html, Properties};

use crate::{app::index::IndexMessage, components::ICON, util::AsClasses};

thread_local! {
    static MESSAGE: StyleSource = css!(r#"
        text-align: center;
        margin-top: 5px;
        margin-right: 20px;
        margin-left: 20px;
        padding: 5px;
        animation: popUp ${at}, pulse 2s infinite;
        animation-delay: 0s, ${at};
        border-radius: 8px;
        transform: scale(1);
        user-select: none;
        overflow: hidden;

        & > span {
            vertical-align: middle;
        }
    "#, at = "1s");

    static ERROR: StyleSource = css!(r#"
        background-color: #d10723;
    "#);

    static WARNING: StyleSource = css!(r#"
        background: #ffbf1d;
        color: black;
    "#);

    static INFO: StyleSource = css!(r#"
        background: #c0c0c0;
        color: black;
    "#);

    static CONTAINER: StyleSource = css!(r#"
        position: fixed;
        left: 0;
        top: 0;
        width: 100%;
        display: flex;
        flex-direction: column;
    "#);
}

#[derive(Clone, Eq, Derivative, Debug)]
#[derivative(PartialEq, Hash)]
pub enum Message {
    Error(AttrValue),
    #[allow(unused)]
    Warning(AttrValue),
    #[allow(unused)]
    Info(AttrValue),
    More(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        i32,
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
            Message::Error(_) => (ERROR.as_classes(), "error"),
            Message::Warning(_) => (WARNING.as_classes(), "warning"),
            Message::Info(_) => (INFO.as_classes(), "info"),
            Message::More(_) => (classes!(), "add_circle"),
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
                <div { onclick } class={ classes!(class, MESSAGE.as_classes()) }>
                    <span class={ ICON.as_classes() }>
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

        let messages = ctx
            .props()
            .messages
            .iter()
            .rev()
            // only display `display_amount` messages at a time
            .take(display_amount);

        let left = ctx.props().messages.len() as i32 - display_amount as i32;
        let msg = Message::More(left);

        let more = (left > 0).then_some(&msg);

        let messages = messages
            .chain(more)
            .map(|m| m.to_html(ctx.props().manage_messages.clone()))
            .collect::<Html>();

        html! {
            <>
                <div class={ CONTAINER.as_classes() }>
                    { messages }
                </div>
            </>
        }
    }
}
