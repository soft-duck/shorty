use enclose::enclose;
use reqwest::Client;
use stylist::{css, StyleSource};
use tracing::{debug, warn};
use validated::Validated;
use yew::{
    html,
    platform::spawn_local,
    AttrValue,
    Callback,
    Component,
    Context,
    Html,
    NodeRef,
    Properties,
};

use super::{
    advanced_mode::AdvancedMode,
    expiration_input::ExpirationInput,
    link_input::{LinkInput, LinkInputMessage},
    TEXT_INPUT,
};
use crate::{
    app::index::IndexMessage,
    endpoint,
    types::{error::RequestError, link_config::LinkConfig, ServerConfig},
    util::{generate_id, AsClasses},
    INPUT_WIDTH,
};

thread_local! {
    static LABEL: StyleSource = css!(r#"
        display: block;
        font-size: 12px;
        margin-bottom: 3px;
        padding-left: 5px;
    "#);

    static CONTAINER: StyleSource = css!(r#"
        flex: 1;
        display: flex;
        flex-direction: column;
        margin-top: 2px;
    "#);

    static EXPIRATION_CONTAINER: StyleSource = css!(r#"
            display: flex;
            flex-direction: column;
            flex: 1;
            min-width: ${iw};
    "#, iw = INPUT_WIDTH);

    static HEADING: StyleSource = css!(r#"
        margin: 0 0 4px;
    "#);
}

async fn make_request(
    link_config: LinkConfig,
    server_config: Option<ServerConfig>,
) -> Result<AttrValue, RequestError> {
    let json = serde_json::to_string(&link_config).expect("Json could not be serialized");

    if let Some(config) = server_config {
        if json.len() > config.max_json_size {
            return Err(RequestError::JsonSizeExceeded);
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

    let text = response
        .text()
        .await
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
                id: link_config
                    .id
                    .expect("Server returned an in use id, even though no custom id was provided"),
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

#[derive(Clone, Debug)]
pub enum LinkFormMessage {
    UpdateState(LinkFormState),
    UpdateServerConfig(ServerConfig),
}

#[derive(Clone, Debug, Default)]
pub enum LinkFormState {
    #[default]
    Input,
    Display(AttrValue),
}

#[derive(Properties, PartialEq)]
pub struct LinkFormPros {
    pub manage_messages: Callback<IndexMessage>,
}

#[derive(Default)]
pub struct LinkForm {
    state: LinkFormState,
    refs: LinkFormRefs,
    server_config: Option<ServerConfig>,
}

impl Component for LinkForm {
    type Message = LinkFormMessage;
    type Properties = LinkFormPros;

    fn create(ctx: &Context<Self>) -> Self {
        let update_server_config = ctx
            .link()
            .callback(|config| LinkFormMessage::UpdateServerConfig(config));

        spawn_local(async move {
            debug!("fetching server config...");

            match reqwest::get(endpoint!("config")).await {
                Ok(response) => match response.json::<ServerConfig>().await {
                    Ok(config) => {
                        debug!("successfully fetched config: {:#?}", config);
                        update_server_config.emit(config);
                    },
                    Err(e) => warn!("failed to parse json server config with: {}", e),
                },
                Err(e) => warn!("fetching server config failed with: {}", e),
            }
        });

        Self {
            state: LinkFormState::Input,
            refs: LinkFormRefs::default(),
            server_config: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LinkFormMessage::UpdateState(s) => self.state = s,
            LinkFormMessage::UpdateServerConfig(config) => self.server_config = Some(config),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let manage_messages = ctx.props().manage_messages.clone();
        let scope = ctx.link().clone();
        let refs = self.refs.clone();

        let onclick = Callback::from(enclose!((self.server_config => s)move |_| {
            let server_config = s.clone();
            let link_config = LinkConfig::try_from(&refs, server_config.clone());

            match link_config {
                Validated::Good(config) => {
                    let manage_messages = manage_messages.clone();

                    debug!("Sending: {:#?}\n to /custom", config);

                    scope.send_future(async move {
                        match make_request(config, server_config.clone()).await {
                            Ok(link) => LinkFormMessage::UpdateState(LinkFormState::Display(link)),
                            Err(e) => {
                                manage_messages.emit(IndexMessage::AddMessage(e.into()));
                                LinkFormMessage::UpdateState(LinkFormState::Input)
                            },
                        }
                    });
                },
                Validated::Fail(errors) => {
                    for error in errors {
                        manage_messages.emit(IndexMessage::AddMessage(error.into()));
                    }
                },
            }
        }));

        let clear_callback = ctx
            .link()
            .callback(|_| LinkFormMessage::UpdateState(LinkFormState::Input));

        // TODO to_string could be optimized
        let maxlength = self
            .server_config
            .as_ref()
            .map(|c| c.max_custom_id_length.to_string());

        let ids = [generate_id(), generate_id(), generate_id()];

        // TODO remove code duplication
        html! {
            <>
                <h1 class={ HEADING.as_classes() }>{ "[WIP] Link Shortener" }</h1>
                <LinkInput { onclick } input_ref={ self.refs.link_input.clone() } message={ LinkInputMessage::from(self.state.clone()) } manage_messages={ ctx.props().manage_messages.clone() } { clear_callback }/>
                <AdvancedMode toggle_ref={ self.refs.advanced_mode.clone() }>
                    <div class={ CONTAINER.as_classes() }>
                        <label class={ LABEL.as_classes() } for={ ids[0].clone() }>{ "Max. usages" }</label>
                        <input id={ ids[0].clone() } class={ TEXT_INPUT.as_classes() } ref={ self.refs.max_usage_input.clone() } type="number" min="0" placeholder=""/>
                    </div>
                    <div class={ CONTAINER.as_classes() }>
                        <label class={ LABEL.as_classes() } for={ ids[1].clone() }>{ "Custom id" }</label>
                        <input id={ ids[1].clone() } class={ TEXT_INPUT.as_classes() } { maxlength } ref={ self.refs.custom_id_input.clone() } type="text" placeholder=""/>
                    </div>
                    <div class={ CONTAINER.as_classes() }>
                        <label class={ LABEL.as_classes() } for={ ids[2].clone() }>{ "Expire after" }</label>
                        <div class={ EXPIRATION_CONTAINER.as_classes() }>
                            <ExpirationInput id={ ids[2].clone() } toggle_ref={ self.refs.expiration_type.clone() } input_ref={ self.refs.expiration_input.clone() }/>
                        </div>
                    </div>
                </AdvancedMode>
            </>
        }
    }
}
