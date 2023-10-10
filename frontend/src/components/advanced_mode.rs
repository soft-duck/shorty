use tracing::debug;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, ResizeObserver};
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
    observer: ResizeObserver,
    _closure: Closure<dyn Fn()>,
}

impl Component for AdvancedMode {
    type Message = AdvancedModeVisibility;
    type Properties = AdvancedModeProps;

    fn create(_: &Context<Self>) -> Self {
        let toggle_container_ref = NodeRef::default();
        let content_ref = NodeRef::default();
        let size_shadow_ref = NodeRef::default();
        let tr = toggle_container_ref.clone();
        let cr = content_ref.clone();
        let ss = size_shadow_ref.clone();
        let closure = Closure::new(move || {
            debug!("Received a resize event");

            let content = cr.cast::<HtmlElement>().unwrap();
            let toggle_container = tr.cast::<HtmlElement>().unwrap();
            let size_shadow = ss.cast::<HtmlElement>().unwrap();

            let content_height = content.scroll_height();
            let toggle_height = toggle_container.scroll_height();

            let height = toggle_height + content_height;

            size_shadow.style().set_property("height", &format!("{}px", height)).unwrap();

        });

        Self {
            visibility: AdvancedModeVisibility::Collapsed,
            content_ref,
            toggle_container_ref,
            size_shadow_ref,
            observer: ResizeObserver::new(closure.as_ref().unchecked_ref()).unwrap(),
            _closure: closure,
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
                <div ref={ self.size_shadow_ref.clone() } class={ classes!("size-shadow") }>
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
        if first_render {
            //TODO is there a better place?
            self.observer.observe(&self.toggle_container_ref.cast::<Element>().unwrap());
        }
        let content = self.content_ref.cast::<HtmlElement>().unwrap();
        let content_height = content.scroll_height();

        if self.visibility == AdvancedModeVisibility::Expanded {
            content.style().set_property("max-height", &format!("{}px", content_height)).unwrap();
        } else {
            content.style().remove_property("max-height").unwrap();
        }
    }
}
