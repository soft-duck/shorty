use stylist::{css, StyleSource};
use web_sys::MouseEvent;
use yew::{html, Component, Context, Html};

use super::about_dialog::AboutDialog;
use crate::{util::AsClasses, ACCENT_COLOR, BACKGROUND_COLOR, FONT_COLOR};

thread_local! {
    static FOOTER: StyleSource = css!(r#"
        justify-content: center;
        display: flex;
        flex-wrap: wrap;

        position: fixed;
        left: 0;
        bottom: 0;

        padding: 0 20px;
        border: ${ac} 1px none;
        border-top-style: solid;

        width: 100%;
        box-sizing: border-box;
        background-color: ${bg};
    "#, ac = ACCENT_COLOR, bg = BACKGROUND_COLOR);

    static FOOTER_ITEM: StyleSource = css!(r#"
        display: block;
        margin: 12px 25px;
        white-space: nowrap;
        text-align: center;
    "#);

    static FOOTER_CONTAINER: StyleSource = css!(r#"
        display: flex;
        flex-wrap: wrap;
    "#);

    // TODO put hover in variable
    static FOOTER_LINK: StyleSource = css!(r#"
        &, &:visited {
            color: ${fc};
            text-decoration: none;
            cursor: pointer;
        }

        &:hover {
            color: lightgray;
        }
    "#, fc = FONT_COLOR);
}

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
        Self { dialog_open: false }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FooterMessage::OpenDialog => {
                self.dialog_open = true;
                true
            },
            FooterMessage::DialogOpened => {
                self.dialog_open = false;
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let html = [
            ("Source", "https://github.com/flamion/shorty"),
            ("Issues", "https://github.com/flamion/shorty/issues"),
            ("Matrix", "https://matrix.to/#/#shorty:matrix.netflam.de"),
        ]
        .map(|(text, href)| {
            html! {
                <>
                    <div class={ FOOTER_ITEM.as_classes() }>
                            <a target="_blank" class={ FOOTER_LINK.as_classes() } { href }>{ text }</a>
                    </div>
                </>
            }
        })
        .into_iter()
        .collect::<Html>();

        let onclick = ctx
            .link()
            .callback(|_: MouseEvent| FooterMessage::OpenDialog);
        let callback = ctx.link().callback(|_| FooterMessage::DialogOpened);

        html! {
            <>
                <AboutDialog open_signal={ callback } open={ self.dialog_open }/>
                <footer class={ FOOTER.as_classes() }>
                    <div class={ FOOTER_CONTAINER.as_classes() }>
                        { html }
                        <div class={ FOOTER_ITEM.as_classes() }>
                            <a { onclick } class={ FOOTER_LINK.as_classes() }>{ "About" }</a>
                        </div>
                    </div>
                </footer>
            </>
        }
    }
}
