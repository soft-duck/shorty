use tracing::debug;
use web_sys::MouseEvent;
use yew::{classes, Component, Context, Html, html};

use super::about_dialog::AboutDialog;

pub struct Footer {
    dialog_open: bool,
}

pub enum FooterMessage {
    OpenDialog,
    DialogOpened,
}

impl Component for Footer {
    type Message = FooterMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            dialog_open: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FooterMessage::OpenDialog => {
                self.dialog_open = true;
                true
            }
            FooterMessage::DialogOpened => {
                self.dialog_open = false;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let html = [
            ("Source", "https://github.com/flamion/shorty"),
            ("Issues", "https://github.com/flamion/shorty/issues"),
        ].map(|(text, href)| {
            html! {
                <>
                    <div class={ classes!("footer-column") }>
                            <a target="_blank" class={ classes!("footer") } { href }>{ text }</a>
                    </div>
                </>
            }
        }).into_iter().collect::<Html>();

        let onclick = ctx.link().callback(|_: MouseEvent| FooterMessage::OpenDialog);
        let callback = ctx.link().callback(|_| FooterMessage::DialogOpened);

        html! {
            <>
                <AboutDialog open_signal={ callback } open={ self.dialog_open }/>
                <footer id={ "footer" }>
                    <div class={ classes!("footer-group") }>
                        { html }
                        <div class={ classes!("footer-column") }>
                            <a { onclick } class={ classes!("footer") }>{ "About" }</a>
                        </div>
                    </div>
                </footer>
            </>
        }
    }
}