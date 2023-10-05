use enclose::enclose;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{AttrValue, Callback, Component, Context, Html, html, InputEvent, NodeRef, Properties};

use crate::util::generate_id;

use super::advanced_mode::AdvancedModeVisibility;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum LabelPosition {
    Left,
    #[default]
    Right,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ToggleState {
    On = 1,
    Off = 0,
}

impl ToggleState {
    pub fn checked(&self) -> bool {
        *self == Self::On
    }
}

impl From<bool> for ToggleState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::On,
            false => Self::Off,
        }
    }
}

// could be replaced with a cast if the enum values are assigned the same values
impl From<AdvancedModeVisibility> for ToggleState {
    fn from(value: AdvancedModeVisibility) -> Self {
        match value {
            AdvancedModeVisibility::Expanded => ToggleState::On,
            AdvancedModeVisibility::Collapsed => ToggleState::Off,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ToggleProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub position: LabelPosition,
    pub callback: Option<Callback<ToggleState>>,
    pub checkbox_ref: NodeRef,
}

pub struct Toggle {
    state: ToggleState,
    id: AttrValue,
}

impl Component for Toggle {
    type Message = ToggleState;
    type Properties = ToggleProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            state: ToggleState::Off,
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

        let label = html! {
                <label for={ self.id.clone() }>{ &ctx.props().label }</label>
        };

        let mut html = vec![html! {
            <input ref={ ctx.props().checkbox_ref.clone() } type="checkbox" checked={ self.state.checked() } id={ self.id.clone() } { oninput }/>
        }];

        if ctx.props().position == LabelPosition::Left {
            html.insert(0, label);
        } else {
            html.push(label);
        }

        html.into_iter().collect()
    }
}