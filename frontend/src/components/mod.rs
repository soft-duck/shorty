use stylist::{css, StyleSource};

use crate::{BACKGROUND_COLOR, FONT_COLOR, INPUT_WIDTH};

mod about_dialog;
mod advanced_mode;
pub mod duration_input;
pub mod expiration_input;
pub mod footer;
pub mod link_form;
mod link_input;
pub mod message_box;
pub mod toggle_input;

thread_local! {
    static ICON: StyleSource = css!(r#"
        font-family: 'Material Symbols Outlined';
        font-weight: normal;
        font-style: normal;
        font-size: 18px;
        display: inline-block;
        line-height: 1;
        text-transform: none;
        letter-spacing: normal;
        word-wrap: normal;
        white-space: nowrap;
        direction: ltr;
    "#);

    // TODO @:focus lighter version of border-color
    // TODO variable for border color
    static TEXT_INPUT: StyleSource = css!(r#"
        padding: 8px;
        border-color: rgb(94, 101, 103);
        font-size: 18px;
        border-radius: 10px;
        border-width: 1px;
        height: 24px;
        color: ${fc};
        background-color: ${bg};
        border-style: solid;
        margin-bottom: 4px;
        min-width: ${iw};

        &:focus {
            outline-style: none;
            border-style: solid;
            border-color: white;
            border-width: 1px;
        }
    "#, bg = BACKGROUND_COLOR, fc = FONT_COLOR, iw = INPUT_WIDTH);
}
