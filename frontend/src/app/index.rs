use ritelinked::LinkedHashSet;
use stylist::{css, StyleSource};
use yew::{html, AttrValue, Component, Context, Html};

use crate::{
    components::{
        footer::Footer,
        link_form::LinkForm,
        message_box::{Message, MessageBox},
    },
    util::AsClasses,
    ACCENT_COLOR,
    BACKGROUND_COLOR,
    FONT_COLOR,
    FONT_FAMILY,
};

pub enum IndexMessage {
    AddMessage(Message),
    RemoveMessage(Message),
    ClearMessages,
}

pub struct Index {
    messages: LinkedHashSet<Message>,
}

thread_local! {
    // TODO dialog min / max width
    static PAGE_CONTAINER: StyleSource = css!(r#"
        display: flex;
        flex-direction: column;
        min-height: 100vh;
        color: ${fc};
        font-family: ${fm};

        & input, button {
            font-family: ${fm};
        }

        & input::-webkit-outer-spin-button,
        & input::-webkit-inner-spin-button {
            -webkit-appearance: none;
            margin: 0;
        }


        & input[type=number] {
            -moz-appearance: textfield;
        }

        & dialog {
            background-color: ${bg};
            color: white;
            border-color: ${ac};
            font-family: ${fm};
            border-width: 1px;
            border-style: solid;
            border-radius: 8px;
            width: 350px;
        }

        & dialog > form {
            display: flex;
            flex-direction: column;
            height: fit-content;
        }

        & input[type=date] {
            text-align: right;
        }

        ::selection, ::-moz-selection {
            color: white;
            background: #DC143C;
        }
    "#, fc = FONT_COLOR, fm = FONT_FAMILY, bg = BACKGROUND_COLOR, ac = ACCENT_COLOR);

    static OUTER_CONTAINER: StyleSource = css!(r#"
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
    "#);

    static INNER_CONTAINER: StyleSource = css!(r#"
        background-color: ${bg};
        border-width: 1px;
        border-color: ${ac};
        border-radius: 30px;
        margin: 10px;
        padding: 25px;
        width: fit-content;
    "#, bg = BACKGROUND_COLOR, ac = ACCENT_COLOR);
}

impl Component for Index {
    type Message = IndexMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let mut messages = LinkedHashSet::new();

        // for i in 0..10 {
        //     let message = AttrValue::from(format!(
        //         "A looooooooooooong message to see css styling effects {}",
        //         i
        //     ));
        //     messages.insert(Message::Error(message.clone()));
        //     messages.insert(Message::Warning(message.clone()));
        //     messages.insert(Message::Info(message));
        // }

        Self { messages }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexMessage::AddMessage(m) => {
                self.messages.insert(m.clone());
            },
            IndexMessage::RemoveMessage(m) => {
                self.messages.remove(&m);
            },
            IndexMessage::ClearMessages => self.messages.clear(),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let manage_messages = ctx.link().callback(|m| m);

        html! {
            <>
                <div class={ PAGE_CONTAINER.as_classes() }>
                    // TODO clone could be mitigated with an appropriate pointer type
                    <MessageBox manage_messages={ manage_messages.clone() } messages={ self.messages.clone() }/>
                    <div class={ OUTER_CONTAINER.as_classes() }>
                        <div class={ INNER_CONTAINER.as_classes() }>
                            <LinkForm { manage_messages } />
                        </div>
                    </div>
                    <Footer/>
                </div>
            </>
        }
    }
}
