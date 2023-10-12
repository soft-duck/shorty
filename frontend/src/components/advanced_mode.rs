use stylist::{css, StyleSource};
use tracing::debug;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Element, HtmlElement, ResizeObserver};
use yew::{Children, classes, Component, Context, html, Html, NodeRef, Properties};

use crate::{ACCENT_COLOR, util::AsClasses};

use super::toggle_input::{ToggleInput, ToggleInputState};

thread_local! {
    // https://www.w3schools.com/howto/howto_js_collapsible.asp
    // TODO split with other parts
    static CONTAINER: StyleSource = css!(r#"
        margin-top: 15px;
        border-color: ${ac};
        border-width: 1px;
        border-radius: 10px;
        border-style: solid;
    "#, ac = ACCENT_COLOR);

    static NODE_CONTAINER: StyleSource = css!(r#"
        display: flex;

        & > * {
            flex: 1;
        }
    "#);

    static NODES_OUTER: StyleSource = css!(r#"
        max-height: 0;
        overflow: hidden;
        transition: max-height .5s;
    "#);

    static NODES_INNER: StyleSource = css!(r#"
        margin: 15px;
        margin-top: 8px;
        display: flex;
        flex-direction: column;
    "#);

    static TOGGLE_CONTAINER: StyleSource = css!(r#"
        display: flex;
        justify-content: center;
    "#);

    static TOGGLE: StyleSource = css!(r#"
        &:is(label) {
            flex: 1;
            text-align: center;
            padding: 8px;
            border-radius: 8px;
            transition: background-color .5s, border-radius .1s;
            user-select: none;
            outline: ${ac} solid 1px;
        }

        &:is(label):hover {
            background-color: #b31234;
        }

        &:is(input[type=checkbox]) {
            display: none;
        }

        &:is(input[type=checkbox]):checked + &:is(label) {
            background-color: ${ac};
            border-radius: 8px 8px 0 0;
            outline: ${ac} solid 1px;
        }

        &:is(input[type=checkbox]):checked + &:is(label):hover {
            background-color: #b31234;
        }
    "#, ac = ACCENT_COLOR);
}

#[derive(Copy, Clone, PartialEq)]
pub enum AdvancedModeVisibility {
    Collapsed,
    Expanded,
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

            size_shadow
                .style()
                .set_property("height", &format!("{}px", height))
                .unwrap();
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
                    <div class={ CONTAINER.as_classes() }>
                        <div ref={ self.toggle_container_ref.clone() } class={ TOGGLE_CONTAINER.as_classes() }>
                            <ToggleInput class={ TOGGLE.as_classes() } checkbox_ref={ ctx.props().toggle_ref.clone() } label="Advanced mode" { callback }/>
                        </div>
                        <div ref={ self.content_ref.clone() } class={ NODES_OUTER.as_classes() }>
                            <div class={ NODES_INNER.as_classes() }>
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
            self.observer
                .observe(&self.toggle_container_ref.cast::<Element>().unwrap());
        }
        let content = self.content_ref.cast::<HtmlElement>().unwrap();
        let content_height = content.scroll_height();

        if self.visibility == AdvancedModeVisibility::Expanded {
            content
                .style()
                .set_property("max-height", &format!("{}px", content_height))
                .unwrap();
        } else {
            content.style().remove_property("max-height").unwrap();
        }
    }
}
