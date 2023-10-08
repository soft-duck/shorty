use web_sys::HtmlDialogElement;
use yew::{Callback, classes, Component, Context, Html, html, NodeRef, Properties};

pub struct AboutDialog {
    dialog_ref: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct AboutDialogProps {
    pub open: bool,
    pub open_signal: Callback<()>,
}

impl Component for AboutDialog {
    type Message = ();
    type Properties = AboutDialogProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            dialog_ref: NodeRef::default(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        /*
            Attribution list:
                - google fonts robot slab
                - google material design icons
                - used libraries?
        */

        html! {
            <dialog ref={ self.dialog_ref.clone() }>
                <form method="dialog">
                    <div class={ classes!("dialog-content-container") }>
                        <h1 class={ classes!("dialog-heading") }>{ "About" }</h1>
                        <p>{
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
                            eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad \
                            minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip \
                            ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
                            voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur \
                            sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt \
                            mollit anim id est laborum."
                        }</p>
                    </div>
                    <div class={ classes!("dialog-button-container") }>
                        <button class={ classes!("dialog-close-button") }>{ "Close" }</button>
                    </div>
                </form>
            </dialog>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
        if ctx.props().open {
            let dialog = self.dialog_ref.cast::<HtmlDialogElement>().unwrap();
            dialog.show_modal().unwrap();
            ctx.props().open_signal.emit(());
        }
    }
}