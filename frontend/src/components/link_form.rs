use reqwest::Client;
use tracing::debug;
use wasm_bindgen::JsCast;
use web_sys::SubmitEvent;
use yew::{html, AttrValue, Callback, Component, Context, Html, NodeRef, Properties};

use super::{
    advanced_mode::AdvancedMode,
    expiration_mode::ExpirationMode,
    link_input::{LinkInput, LinkInputMessage},
    message_box::Message,
};
use crate::{endpoint, types::link_config::LinkConfig};

#[derive(Default, Clone)]
pub struct LinkFormRefs {
    pub link_input: NodeRef,
    pub advanced_mode: NodeRef,
    pub max_usage_input: NodeRef,
    pub custom_id_input: NodeRef,
    pub expiration_input: NodeRef,
    pub expiration_type: NodeRef,
}

#[derive(Clone, Debug, Default)]
pub enum LinkFormMessage {
    #[default]
    Input,
    Display(AttrValue),
}

#[derive(Properties, PartialEq)]
pub struct LinkFormPros {
    pub callback: Callback<Message>,
}

#[derive(Default)]
pub struct LinkForm {
    state: LinkFormMessage,
    refs: LinkFormRefs,
}

impl Component for LinkForm {
    type Message = LinkFormMessage;
    type Properties = LinkFormPros;

    fn create(_: &Context<Self>) -> Self {
        Self {
            state: LinkFormMessage::Input,
            refs: LinkFormRefs::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        self.state = msg;

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let display = ctx.props().callback.clone();
        let scope = ctx.link().clone();
        let refs = self.refs.clone();

        let callback = Callback::from(move |event: SubmitEvent| {
            let link_config = LinkConfig::try_from(&refs).unwrap();

            debug!("Sending: {:#?}\n to /custom", link_config);
            let d = display.clone();

            scope.send_future(async move {
                let client = Client::new();
                let result = client
                    .post(endpoint!("custom"))
                    .json(&link_config)
                    .send()
                    .await;

                // api docs do not mention whats returned after a simple post
                let response = result.unwrap();
                let status = response.status();

                let text = response.text().await.unwrap();

                debug!(
                    "Received: {:#?}\n from /custom with code {}",
                    text,
                    status.as_u16()
                );

                if status.is_success() {
                    LinkFormMessage::Display(AttrValue::from(text))
                } else {
                    d.emit(Message::Error(AttrValue::from(format!(
                        "[Temporary] Got '{}' with code {}",
                        text,
                        status.as_u16()
                    ))));
                    LinkFormMessage::Input
                }
            });

            event.prevent_default();
        });

        let clear_callback = ctx.link().callback(|_| LinkFormMessage::Input);

        html! {
            <>
                <form onsubmit={ callback }>
                    <LinkInput input_ref={ self.refs.link_input.clone() } message={ LinkInputMessage::from(self.state.clone()) } { clear_callback }/>
                    <AdvancedMode toggle_ref={ self.refs.advanced_mode.clone() }>
                        <input ref={ self.refs.max_usage_input.clone() } type="number" min="0" placeholder="Maximum usages"/>
                        <input ref={ self.refs.custom_id_input.clone() } type="text" placeholder="Custom alphanumeric id"/>
                        <ExpirationMode toggle_ref={ self.refs.expiration_type.clone() } input_ref={ self.refs.expiration_input.clone() }/>
                    </AdvancedMode>
                </form>
            </>
        }
    }
}
