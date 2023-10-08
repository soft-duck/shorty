use std::future::Future;

use reqwest::Client;
use tracing::debug;
use validated::Validated;
use web_sys::KeyboardEvent;
use yew::{AttrValue, Callback, classes, Component, Context, html, Html, NodeRef, Properties};

use crate::{endpoint, types::link_config::LinkConfig};
use crate::types::error::RequestError;
use crate::util::server_config;

use super::{
    advanced_mode::AdvancedMode,
    expiration_input::ExpirationInput,
    link_input::{LinkInput, LinkInputMessage},
    message_box::Message,
};

async fn make_request(link_config: LinkConfig) -> Result<AttrValue, RequestError> {
    let json = serde_json::to_string(&link_config)
        .expect("Json could not be serialized");

    if let Some(config) = server_config() {
        if json.len() > config.max_json_size {
            return Err(RequestError::JsonSizeExceeded)
        }
    }

    let client = Client::new();
    let result = client
        .post(endpoint!("custom"))
        .header("content-type", "application/json")
        .body(json)
        .send()
        .await;

    if let Err(e) = result {
        return Err(RequestError::UnsuccessfulRequest { error: e });
    }

    let response = result.unwrap();
    let status = response.status();

    let text = response.text().await
        .expect("Expected a text/plain response");

    debug!(
        "Received: {:#?}\n from /custom with code {}",
        text,
        status.as_u16()
    );

    if status.is_success() {
        Ok(AttrValue::from(text))
    } else {
        Err(match status.as_u16() {
            // TODO use concrete backend error enum to match
            400 => RequestError::Backend400,
            409 => RequestError::IdInUse {
                id: link_config.id
                    .expect("Server returned an in use id, even though no custom id was provided")
            },
            code => panic!("Unexpected return code: {}", code),
        })
    }
}

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

        let onclick = Callback::from(move |_| {
            let link_config = LinkConfig::try_from(&refs);

            match link_config {
                Validated::Good(config) => {
                    let display = display.clone();

                    debug!("Sending: {:#?}\n to /custom", config);

                    scope.send_future(async move {
                        match make_request(config).await {
                            Ok(link) => LinkFormMessage::Display(link),
                            Err(e) => {
                                display.emit(e.into());
                                LinkFormMessage::Input
                            }
                        }
                    });
                },
                Validated::Fail(errors) => {
                    for error in errors {
                        display.emit(error.into());
                    }
                },
            }
        });

        let clear_callback = ctx.link().callback(|_| LinkFormMessage::Input);
        // TODO rerender if server_config is fetched or not if it failed
        // let size = server_config().map(|c| c.max_custom_id_length.to_string());

        html! {
            <>
                <h1 class={ classes!("heading") }>{ "[WIP] Link Shortener" }</h1>
                <LinkInput { onclick } input_ref={ self.refs.link_input.clone() } message={ LinkInputMessage::from(self.state.clone()) } { clear_callback }/>
                <AdvancedMode toggle_ref={ self.refs.advanced_mode.clone() }>
                    <div>
                        <input class={ classes!("input-box") } ref={ self.refs.max_usage_input.clone() } type="number" min="0" placeholder="Maximum usages"/>
                    </div>
                    <div>
                        <input class={ classes!("input-box") } ref={ self.refs.custom_id_input.clone() } type="text" placeholder="Custom alphanumeric id"/>
                    </div>
                    <div class={ classes!("expiration-mode-container") }>
                        <ExpirationInput toggle_ref={ self.refs.expiration_type.clone() } input_ref={ self.refs.expiration_input.clone() }/>
                    </div>
                </AdvancedMode>
            </>
        }
    }
}
