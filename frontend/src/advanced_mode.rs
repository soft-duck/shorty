use yew::{AttrValue, Children, Component, Context, Html, html, NodeRef, Properties};

use crate::toggle::{LabelPosition, Toggle, ToggleState};

#[derive(Copy, Clone, PartialEq)]
pub enum AdvancedModeVisibility {
    Collapsed,
    Expanded,
}

impl AdvancedModeVisibility {
    fn style(&self) -> AttrValue {
        match self {
            AdvancedModeVisibility::Collapsed => AttrValue::from("visibility: collapse;"),
            AdvancedModeVisibility::Expanded => AttrValue::from("visibility: visible;")
        }
    }
}

pub struct AdvancedMode {
    visibility: AdvancedModeVisibility,
}

#[derive(Properties, PartialEq)]
pub struct AdvancedModeProps {
    #[prop_or_default]
    pub children: Children,
    pub toggle_ref: NodeRef,
}

impl From<ToggleState> for AdvancedModeVisibility {
    fn from(value: ToggleState) -> Self {
        match value {
            ToggleState::On => AdvancedModeVisibility::Expanded,
            ToggleState::Off => AdvancedModeVisibility::Collapsed,
        }
    }
}


impl Component for AdvancedMode {
    type Message = AdvancedModeVisibility;
    type Properties = AdvancedModeProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            visibility: AdvancedModeVisibility::Collapsed
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        if self.visibility == msg {
            return false;
        }

        self.visibility = msg;

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link()
            .callback(AdvancedModeVisibility::from);

        let elements = ctx.props().children.iter().map(|f| {
            html! {
                <>
                    <div>
                        { f.clone() }
                    </div>
                </>
            }
        }).collect::<Html>();

        html! {
            <>
                <div>
                    <Toggle checkbox_ref={ ctx.props().toggle_ref.clone() } label="Advanced mode" position={ LabelPosition::Left } { callback }/>
                </div>
                <div style={ self.visibility.style() }>
                    { elements }
                </div>
            </>
        }
    }
}