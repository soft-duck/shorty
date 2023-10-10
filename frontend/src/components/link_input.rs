use gloo_timers::callback::Timeout;
use tracing::debug;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{html, AttrValue, Callback, Component, Context, Html, NodeRef, Properties, classes, Classes};
use crate::app::index::IndexMessage;

use super::link_form::LinkFormMessage;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LinkInputState {
    Copied,
    Copy,
}

impl LinkInputState {
    fn class(&self) -> Classes {
        classes!(match self {
            LinkInputState::Copied => "copied",
            LinkInputState::Copy => "copy",
        })
    }
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
    pub manage_messages: Callback<IndexMessage>,
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

            // FIXME handle promise and unwrap
            let c = web_sys::window().unwrap().navigator().clipboard().unwrap();
            let _ = c.write_text(&link);
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut onclick = None;

        let mut text = "Shorten";
        let mut content = None;
        let mut oninput = None;
        let mut classes = Some(self.state.class());

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

                    let input = self.input_ref.cast::<HtmlInputElement>().expect(&format!(
                        "Expected {:?} to be an HtmlInputElement",
                        self.input_ref
                    ));

                    let input_len = input.value().len() as u32;

                    // TODO should this stay?
                    // ignore results, if it does not work we do not care
                    let _ = input.focus();
                    let _ = input.set_selection_start(Some(0));
                    let _ = input.set_selection_end(Some(input_len));
                },
            }
        } else {
            classes = None;
            let manage_messages = ctx.props().manage_messages.clone();
            onclick = Some(ctx.props().onclick.reform(move |event| {
                manage_messages.emit(IndexMessage::ClearMessages);
                event
            }));
        }

        html! {
            <>
                <div class={ classes!("link-input-container") }>
                    <input class={ classes!("input-box", "link-input") } ref={ self.input_ref.clone() } type="text" value={ content } oninput={ oninput } placeholder="Put a link to shorten here!"/>
                    <button class={ classes!("shorten-button", classes) } type={ "button" } onclick={ onclick }>{ text }</button>
                </div>
            </>
        }
    }
}
