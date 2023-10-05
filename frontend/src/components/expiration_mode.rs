use strum_macros::Display;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use yew::{Component, Context, Html, html, NodeRef, Properties};

use crate::util::try_get_local_offset;

use super::duration_input::DurationInput;
use super::toggle::{LabelPosition, Toggle, ToggleState};

pub struct ExpirationMode {
    input_type: ExpirationType,
}

#[derive(Properties, PartialEq)]
pub struct ExpirationModeProps {
    pub input_ref: NodeRef,
    pub toggle_ref: NodeRef,
}

#[derive(Copy, Clone, PartialEq, Display)]
pub enum ExpirationType {
    Date,
    Duration,
}

impl From<ToggleState> for ExpirationType {
    fn from(value: ToggleState) -> Self {
        match value {
            ToggleState::On => ExpirationType::Date,
            ToggleState::Off => ExpirationType::Duration,
        }
    }
}

impl ExpirationType {
    fn flipped(&self) -> Self {
        match self {
            Self::Date => Self::Duration,
            Self::Duration => Self::Date,
        }
    }
}

impl Component for ExpirationMode {
    type Message = ExpirationType;
    type Properties = ExpirationModeProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            input_type: ExpirationType::from(ToggleState::Off),
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

                <Toggle checkbox_ref={ ctx.props().toggle_ref.clone() } label={ self.input_type.flipped().to_string() } position={ LabelPosition::Right } { callback }/>
            </>
        }
    }
}