use gloo_timers::callback::Timeout;
use tracing::debug;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{AttrValue, Callback, Component, Context, html, Html, NodeRef, Properties};

use super::link_form::LinkFormMessage;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LinkInputState {
    Copied,
    Copy,
}

#[derive(Default, PartialEq, Debug)]
pub enum LinkInputMessage {
    Update {
        link: AttrValue,
    },
    #[default]
    Clear,
    UpdateState(LinkInputState),
}

impl From<LinkFormMessage> for LinkInputMessage {
    fn from(value: LinkFormMessage) -> Self {
        match value {
            LinkFormMessage::Input => Self::Clear,
            LinkFormMessage::Display(m) => Self::Update { link: m },
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkInputProps {
    pub message: LinkInputMessage,
    pub clear_callback: Callback<()>,
    pub input_ref: NodeRef,
    pub onclick: Callback<MouseEvent>,
}

pub struct LinkInput {
    state: LinkInputState,
    input_ref: NodeRef,
}

impl Component for LinkInput {
    type Message = LinkInputMessage;
    type Properties = LinkInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: LinkInputState::Copy,
            input_ref: ctx.props().input_ref.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LinkInputMessage::Clear => ctx.props().clear_callback.emit(()),
            LinkInputMessage::UpdateState(s) => self.state = s,
            _ => (),
        }

        if let LinkInputMessage::UpdateState(LinkInputState::Copied) = msg {
            let link = self.input_ref.cast::<HtmlInputElement>().unwrap().value();
            debug!("copying '{}' to clipboard", link);

            // let clipboard = window().navigator().clipboard().unwrap();
            // FIXME handle promise and unwrap
            let c = web_sys::window().unwrap().navigator().clipboard().unwrap();
            c.write_text(&link);
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut onclick = None;

        let mut text = "Shorten";
        let mut content = None;
        let mut oninput = None;

        if let LinkInputMessage::Update { link } = &ctx.props().message {
            content = Some(link.clone());
            oninput = Some(ctx.link().callback(|_| LinkInputMessage::Clear));

            match self.state {
                LinkInputState::Copy => {
                    text = "Copy";
                    onclick = Some(
                        ctx.link()
                            .callback(|_| LinkInputMessage::UpdateState(LinkInputState::Copied)),
                    );
                },
                LinkInputState::Copied => {
                    text = "Copied";
                    let reset = ctx
                        .link()
                        .callback(|_| LinkInputMessage::UpdateState(LinkInputState::Copy));

                    let timeout = Timeout::new(1_000, move || {
                        reset.emit(());
                    });

                    timeout.forget();
                },
            }
        } else {
            onclick = Some(ctx.props().onclick.clone());
        }

        html! {
            <>
                <input ref={ self.input_ref.clone() } type="text" value={ content } oninput={ oninput } placeholder="Put a link to shorten here!"/>
                <button type={ "button" } onclick={ onclick }>{ text }</button>
            </>
        }
    }
}
