use strum_macros::Display;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use yew::{html, Component, Context, Html, NodeRef, Properties};

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
}

impl From<ToggleInputState> for ExpirationType {
    fn from(value: ToggleInputState) -> Self {
        match value {
            ToggleInputState::On => ExpirationType::Date,
            ToggleInputState::Off => ExpirationType::Duration,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ExpirationModeProps {
    pub input_ref: NodeRef,
    pub toggle_ref: NodeRef,
}

pub struct ExpirationMode {
    input_type: ExpirationType,
}

impl Component for ExpirationMode {
    type Message = ExpirationType;
    type Properties = ExpirationModeProps;

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
        let today = OffsetDateTime::now_utc()
            .to_offset(try_get_local_offset())
            .date()
            .format(&Iso8601::DATE)
            .unwrap();

        html! {
            <>
                if self.input_type == ExpirationType::Date {
                    <input min={ today } ref={ ctx.props().input_ref.clone() } type="date"/>
                } else {
                    <DurationInput input_ref={ ctx.props().input_ref.clone() }/>
                }

                <ToggleInput checkbox_ref={ ctx.props().toggle_ref.clone() } label={ self.input_type.flipped().to_string() } position={ LabelPosition::Right } { callback }/>
            </>
        }
    }
}
