use strum_macros::Display;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use yew::{html, Component, Context, Html, NodeRef, Properties, classes, AttrValue};

use super::{
    duration_input::DurationInput,
    toggle_input::{LabelPosition, ToggleInput, ToggleInputState},
};
use crate::util::try_get_local_offset;

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
            ExpirationType::Date => ("date", "calendar_month"),
            ExpirationType::Duration => ("duration", "schedule"),
        };

        html! {
            <span>
                <span class={ classes!("material-symbols-outlined", "icon-size", icon.0) }>{ icon.1 }</span>
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
                    <input id={ ctx.props().id.clone() } class={ classes!("input-box", "expiration-input") } min={ today } ref={ ctx.props().input_ref.clone() } type="date"/>
                } else {
                    <DurationInput id={ ctx.props().id.clone() } class={ classes!("expiration-input") } input_ref={ ctx.props().input_ref.clone() }/>
                }

                <ToggleInput class={ classes!("expiration-mode-toggle") } checkbox_ref={ ctx.props().toggle_ref.clone() } label={ self.input_type.flipped().html() } position={ LabelPosition::Right } { callback }/>
            </>
        }
    }
}
