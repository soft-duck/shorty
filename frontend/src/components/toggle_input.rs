use enclose::enclose;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{
    classes,
    html,
    AttrValue,
    Callback,
    Classes,
    Component,
    Context,
    Html,
    InputEvent,
    NodeRef,
    Properties,
};

use super::advanced_mode::AdvancedModeVisibility;
use crate::util::generate_id;

#[derive(Copy, Clone, PartialEq)]
pub enum ToggleInputState {
    On = 1,
    Off = 0,
}

impl ToggleInputState {
    pub fn checked(&self) -> bool {
        *self == Self::On
    }
}

impl From<bool> for ToggleInputState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::On,
            false => Self::Off,
        }
    }
}

// could be replaced with a cast if the enum values are assigned the same values
impl From<AdvancedModeVisibility> for ToggleInputState {
    fn from(value: AdvancedModeVisibility) -> Self {
        match value {
            AdvancedModeVisibility::Expanded => ToggleInputState::On,
            AdvancedModeVisibility::Collapsed => ToggleInputState::Off,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ToggleInputProps {
    pub label: Html,
    pub callback: Option<Callback<ToggleInputState>>,
    pub checkbox_ref: NodeRef,
    #[prop_or_default]
    pub class: Option<Classes>,
}

pub struct ToggleInput {
    state: ToggleInputState,
    id: AttrValue,
}

impl Component for ToggleInput {
    type Message = ToggleInputState;
    type Properties = ToggleInputProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            state: ToggleInputState::Off,
            id: AttrValue::from(generate_id()),
        }
    }

    fn update(&mut self, _: &Context<Self>, state: Self::Message) -> bool {
        self.state = state;

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput: Callback<InputEvent> = ctx.link().callback(enclose! {
            (ctx.props().callback => c) move |e: InputEvent| {
                let state = e.target().unwrap()
                    .dyn_into::<HtmlInputElement>().unwrap()
                    .checked().into();

                c.as_ref().map(|c| c.emit(state));

                state
            }
        });

        html! {
            <>
                <input class={ classes!(ctx.props().class.clone()) } ref={ ctx.props().checkbox_ref.clone() } type="checkbox" checked={ self.state.checked() } id={ self.id.clone() } { oninput }/>
                <label class={ classes!(ctx.props().class.clone()) } for={ self.id.clone() }>{ ctx.props().label.clone() }</label>
            </>
        }
    }
}
