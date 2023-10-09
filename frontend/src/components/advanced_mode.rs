use tracing::debug;
use web_sys::HtmlElement;
use yew::{html, AttrValue, Children, Component, Context, Html, NodeRef, Properties, classes, Classes};

use super::toggle_input::{LabelPosition, ToggleInput, ToggleInputState};

#[derive(Copy, Clone, PartialEq)]
pub enum AdvancedModeVisibility {
    Collapsed,
    Expanded,
}

impl AdvancedModeVisibility {
    fn style(&self) -> AttrValue {
        match self {
            AdvancedModeVisibility::Collapsed => AttrValue::from("visibility: hidden;"),
            AdvancedModeVisibility::Expanded => AttrValue::from("visibility: visible;"),
        }
    }

    fn class(&self) -> Classes {
        match self {
            AdvancedModeVisibility::Collapsed => classes!(),
            AdvancedModeVisibility::Expanded => classes!("expanded"),
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
    content_ref: NodeRef,
    toggle_container_ref: NodeRef,
    size_shadow_ref: NodeRef,
    old_heights: (i32, i32)
}

impl Component for AdvancedMode {
    type Message = AdvancedModeVisibility;
    type Properties = AdvancedModeProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            visibility: AdvancedModeVisibility::Collapsed,
            content_ref: NodeRef::default(),
            toggle_container_ref: NodeRef::default(),
            size_shadow_ref: NodeRef::default(),
            old_heights: (0, 0),
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
                        <div class={ classes!("advanced-mode-node-container") }>
                            { f.clone() }
                        </div>
                    </>
                }
            })
            .collect::<Html>();

        html! {
            <>
                <div ref={ self.size_shadow_ref.clone() }>
                    <div class={ classes!("advanced-mode-container") }>
                        <div ref={ self.toggle_container_ref.clone() } class={ classes!("advanced-mode-toggle-container") }>
                            <ToggleInput class={ classes!("advanced-mode-toggle") } checkbox_ref={ ctx.props().toggle_ref.clone() } label="Advanced mode" position={ LabelPosition::Right } { callback }/>
                        </div>
                        <div ref={ self.content_ref.clone() } class={ classes!("advanced-mode-nodes-outer-container"/*, self.visibility.class()*/) }>
                            <div class={ classes!("advanced-mode-nodes-inner-container") }>
                                { elements }
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    // https://www.w3schools.com/howto/howto_js_collapsible.asp
    // TODO remove unwraps
    fn rendered(&mut self, _: &Context<Self>, first_render: bool) {
        // let outer_container = self.outer_container_ref.cast::<HtmlElement>().unwrap();
        //
        // debug!("{}", outer_container.scroll_height());

        let content = self.content_ref.cast::<HtmlElement>().unwrap();
        let toggle_container = self.toggle_container_ref.cast::<HtmlElement>().unwrap();
        let size_shadow = self.size_shadow_ref.cast::<HtmlElement>().unwrap();

        let mut content_height = content.scroll_height();
        let mut toggle_height = toggle_container.scroll_height();

        // only needed because scroll_height is rounded, this mitigates rounding fluctuations
        // https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight#problems_and_solutions
        if first_render {
            self.old_heights = (content_height, toggle_height);
        } else {
            if (content_height - self.old_heights.0).abs() > 1 {
                self.old_heights.0 = content_height;
            } else {
                content_height = self.old_heights.0;
            }

            if (toggle_height - self.old_heights.1).abs() > 1 {
                self.old_heights.1 = toggle_height;
            } else {
                toggle_height = self.old_heights.1;
            }
        }

        debug!("{}", content_height);

        let height = toggle_height + content_height;

        size_shadow.style().set_property("height", &format!("{}px", height)).unwrap();

        if self.visibility == AdvancedModeVisibility::Expanded {
            content.style().set_property("max-height", &format!("{}px", content_height)).unwrap();
        } else {
            content.style().remove_property("max-height").unwrap();
        }

        // debug!("{}", content.scroll_height());
    }

    // fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
    //     // TODO make an except
    //     let content = self.content_ref.cast::<HtmlElement>().unwrap();
    //     // debug!("reached rendered");
    //     // TODO if this works add to message enum to prevent bugs
    //     if self.visibility == AdvancedModeVisibility::Expanded && self.scroll_height.is_none() {
    //         let scroll_height = content.scroll_height();
    //         self.scroll_height = Some(scroll_height);
    //         ctx.link().send_message(AdvancedModeVisibility::Expanded)
    //         // debug!("{}", content.class_list().to_string());
    //         // let style = content.style();
    //
    //         // style.remove_property("max-height").unwrap();
    //         // style.set_property("max-height", &scroll_height).unwrap();
    //     }
    // }
}
