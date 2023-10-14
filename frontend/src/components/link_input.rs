use gloo_timers::callback::Timeout;
use stylist::{css, StyleSource};
use tracing::debug;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{
    classes,
    html,
    AttrValue,
    Callback,
    Classes,
    Component,
    Context,
    Html,
    NodeRef,
    Properties,
};

use super::TEXT_INPUT;
use crate::{
    app::index::IndexMessage,
    components::link_form::LinkFormState,
    util::AsClasses,
    ACCENT_COLOR,
    FONT_COLOR,
};

thread_local! {
    // TODO make variable
    static COPY: StyleSource = css!(r#"
        background-color: #2222ca;

        &:hover {
            background-color: #1b1b9e;
        }
    "#);

    // TODO make variable
    static COPIED: StyleSource = css!(r#"
        background-color: #1dd320;
    "#);

    // TODO make variable
    static BUTTON: StyleSource = css!(r#"
        background-color: ${ac};
        color: ${fc};
        padding: 8px;
        border: none;
        border-radius: 10px;
        font-size: 18px;
        height: 40px;
        user-select: none;
        min-width: 84px;

        &:hover {
            background-color: #b31234;
        }
    "#, ac = ACCENT_COLOR, fc = FONT_COLOR);

    static LINK_INPUT: StyleSource = css!(r#"
        margin-bottom: 0;
        margin-right: 5px;
    "#);

    static CONTAINER: StyleSource = css!(r#"
        white-space: nowrap;
    "#);
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LinkInputState {
    Copied,
    Copy,
}

impl LinkInputState {
    fn class(&self) -> Classes {
        match self {
            LinkInputState::Copied => COPIED.as_classes(),
            LinkInputState::Copy => COPY.as_classes(),
        }
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

impl From<LinkFormState> for LinkInputMessage {
    fn from(value: LinkFormState) -> Self {
        match value {
            LinkFormState::Input => Self::Clear,
            LinkFormState::Display(m) => Self::Update { link: m },
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
                <div class={ CONTAINER.as_classes() }>
                    <input class={ classes!(TEXT_INPUT.as_classes(), LINK_INPUT.as_classes()) } ref={ self.input_ref.clone() } type="text" value={ content } oninput={ oninput } placeholder="Put a link to shorten here!"/>
                    <button class={ classes!(BUTTON.as_classes(), classes) } type={ "button" } onclick={ onclick }>{ text }</button>
                </div>
            </>
        }
    }
}
