use yew::{html, AttrValue, Children, Component, Context, Html, NodeRef, Properties};

use super::toggle_input::{LabelPosition, ToggleInput, ToggleInputState};

#[derive(Copy, Clone, PartialEq)]
pub enum AdvancedModeVisibility {
    Collapsed,
    Expanded,
}

impl AdvancedModeVisibility {
    fn style(&self) -> AttrValue {
        match self {
            AdvancedModeVisibility::Collapsed => AttrValue::from("visibility: collapse;"),
            AdvancedModeVisibility::Expanded => AttrValue::from("visibility: visible;"),
        }
    }
}

impl From<ToggleInputState> for AdvancedModeVisibility {
    fn from(value: ToggleInputState) -> Self {
        match value {
            ToggleInputState::On => AdvancedModeVisibility::Expanded,
            ToggleInputState::Off => AdvancedModeVisibility::Collapsed,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AdvancedModeProps {
    #[prop_or_default]
    pub children: Children,
    pub toggle_ref: NodeRef,
}

pub struct AdvancedMode {
    visibility: AdvancedModeVisibility,
}

impl Component for AdvancedMode {
    type Message = AdvancedModeVisibility;
    type Properties = AdvancedModeProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            visibility: AdvancedModeVisibility::Collapsed,
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
        let callback = ctx.link().callback(AdvancedModeVisibility::from);

        let elements = ctx
            .props()
            .children
            .iter()
            .map(|f| {
                html! {
                    <>
                        <div>
                            { f.clone() }
                        </div>
                    </>
                }
            })
            .collect::<Html>();

        html! {
            <>
                <div>
                    <ToggleInput checkbox_ref={ ctx.props().toggle_ref.clone() } label="Advanced mode" position={ LabelPosition::Left } { callback }/>
                </div>
                <div style={ self.visibility.style() }>
                    { elements }
                </div>
            </>
        }
    }
}
