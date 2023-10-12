use strum_macros::Display;
use stylist::{css, StyleSource};
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use yew::{AttrValue, classes, Component, Context, html, Html, NodeRef, Properties};

use crate::{
    ACCENT_COLOR,
    components::{ICON, TEXT_INPUT},
    INPUT_WIDTH,
    util::{AsClasses, try_get_local_offset},
};

use super::{
    duration_input::DurationInput,
    toggle_input::{ToggleInput, ToggleInputState},
};

thread_local! {
    // because icon is not aligned with text
    static DATE_ICON_HEIGHT: StyleSource = css!(r#"
        vertical-align: -3px;
    "#);

    // because icon is not aligned with text
    static CLOCK_ICON_HEIGHT: StyleSource = css!(r#"
        vertical-align: -4px;
    "#);

    static EXPIRATION_INPUT: StyleSource = css!(r#"
        border-bottom-left-radius: 0;
        border-bottom-right-radius: 0;
    "#);

    // TODO find better way than that calculation
    static TOGGLE: StyleSource = css!(r#"
        &:is(label) {
            user-select: none;
        }

        &:is(label) > span {
            padding-top: 2px;
            padding-bottom: 4px;
            background-color: ${ac};
            border-radius: 0 0 8px 8px;
            min-width: ${iw} + 16 + 2;
            text-align: center;
            display: block;
        }

        &:is(label):hover > span {
            background-color: #b31234;
        }

        &:is(input[type=checkbox]) {
            display: none;
        }
    "#, ac = ACCENT_COLOR, iw = INPUT_WIDTH);
}

#[derive(Copy, Clone, PartialEq, Display)]
pub enum ExpirationType {
    Date,
    Duration,
}

impl ExpirationType {
    fn flipped(&self) -> Self {
        match self {
            Self::Date => Self::Duration,
            Self::Duration => Self::Date,
        }
    }

    fn html(&self) -> Html {
        let icon = match self {
            ExpirationType::Date => (DATE_ICON_HEIGHT.as_classes(), "calendar_month"),
            ExpirationType::Duration => (CLOCK_ICON_HEIGHT.as_classes(), "schedule"),
        };

        html! {
            <span>
                <span class={ classes!(ICON.as_classes(), icon.0) }>{ icon.1 }</span>
                { " " }{ self.to_string() }
            </span>
        }
    }
}

impl From<ToggleInputState> for ExpirationType {
    fn from(value: ToggleInputState) -> Self {
        match value {
            ToggleInputState::On => ExpirationType::Date,
            ToggleInputState::Off => ExpirationType::Duration,
        }
    }
}

impl From<bool> for ExpirationType {
    fn from(value: bool) -> Self {
        Self::from(ToggleInputState::from(value))
    }
}

#[derive(Properties, PartialEq)]
pub struct ExpirationInputProps {
    pub input_ref: NodeRef,
    pub toggle_ref: NodeRef,
    pub id: Option<AttrValue>,
}

pub struct ExpirationInput {
    input_type: ExpirationType,
}

impl Component for ExpirationInput {
    type Message = ExpirationType;
    type Properties = ExpirationInputProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            input_type: ExpirationType::from(ToggleInputState::Off),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        self.input_type = msg;

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(ExpirationType::from);
        let mut today = OffsetDateTime::now_utc();

        // TODO dispatch a warning
        if let Some(offset) = try_get_local_offset() {
            today = today.to_offset(offset);
        }

        let today = today.date().format(&Iso8601::DATE).unwrap();

        html! {
            <>
                if self.input_type == ExpirationType::Date {
                    <input id={ ctx.props().id.clone() } class={ classes!(TEXT_INPUT.as_classes(), EXPIRATION_INPUT.as_classes()) } min={ today } ref={ ctx.props().input_ref.clone() } type="date"/>
                } else {
                    <DurationInput id={ ctx.props().id.clone() } class={ EXPIRATION_INPUT.as_classes() } input_ref={ ctx.props().input_ref.clone() }/>
                }

                <ToggleInput class={ TOGGLE.as_classes() } checkbox_ref={ ctx.props().toggle_ref.clone() } label={ self.input_type.flipped().html() } { callback }/>
            </>
        }
    }
}
