use serde::Serialize;
use time::{format_description::well_known::Iso8601, macros::time, Date, OffsetDateTime};
use web_sys::HtmlInputElement;

use crate::{
    components::{expiration_mode::ExpirationType, link_form::LinkFormRefs, toggle_input::ToggleInputState},
    util::try_get_local_offset,
};

#[derive(Debug, Serialize)]
pub struct LinkConfig {
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<i64>,
    // could be u32 if https://github.com/flamion/shorty/issues/51 is resolved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<i64>,
}

type ParseError = ();

impl LinkConfig {
    fn parse_id(refs: &LinkFormRefs) -> Result<Option<String>, ParseError> {
        let mut id = None;

        if let Some(id_input) = refs.custom_id_input.cast::<HtmlInputElement>() {
            if !id_input.value().is_empty() {
                id = Some(id_input.value())
            }
        }

        Ok(id)
    }

    fn parse_max_uses(refs: &LinkFormRefs) -> Result<Option<i64>, ParseError> {
        let mut max_uses = None;

        if let Some(max_usages_input) = refs.max_usage_input.cast::<HtmlInputElement>() {
            // TODO Handle this
            if let Ok(value) = max_usages_input.value().parse::<i64>() {
                // TODO should 0 uses be allowed?
                if value > 0 {
                    max_uses = Some(value)
                } else {
                    return Err(());
                }
            }
        }

        Ok(max_uses)
    }

    fn parse_date(refs: &LinkFormRefs) -> Result<Option<i64>, ParseError> {
        let mut valid_for = None;

        if let Some(expiration_input) = refs.expiration_input.cast::<HtmlInputElement>() {
            if !expiration_input.value().is_empty() {
                // TODO handle this
                if let Ok(date) = Date::parse(&expiration_input.value(), &Iso8601::DATE) {
                    let offset = try_get_local_offset();

                    let date_time = date.with_time(time!(00:00)).assume_offset(offset);

                    let local_now = OffsetDateTime::now_utc().to_offset(offset);

                    // TODO should negative values be allowed?
                    let mut difference = date_time.unix_timestamp() - local_now.unix_timestamp();
                    // because the timestamp is needed in milliseconds
                    difference *= 1000;

                    valid_for = Some(difference);
                }
            }
        }

        Ok(valid_for)
    }

    fn parse_duration(refs: &LinkFormRefs) -> Result<Option<i64>, ParseError> {
        let mut valid_for = None;

        if let Some(expiration_input) = refs.expiration_input.cast::<HtmlInputElement>() {
            let values = expiration_input.value()
                .split(':')
                // for now can be negative
                .map(|n| n.parse::<i64>())
                .collect::<Result<Vec<_>, _>>();

            if let Ok(mut values) = values {
                if values.len() <= 3 {
                    values.reverse();
                    values.resize(3, 0);
                    values.reverse();

                    valid_for = Some((values[0] * 60 * 60 + values[1] * 60 + values[2]) * 1000);
                }
            }
        }

        Ok(valid_for)
    }
}

impl TryFrom<&LinkFormRefs> for LinkConfig {
    type Error = ParseError;

    // TODO incorporate server config errors
    // TODO when https://github.com/flamion/shorty/issues/51 is resolved apply the solution
    fn try_from(refs: &LinkFormRefs) -> Result<Self, Self::Error> {
        let link = if let Some(link_input) = refs.link_input.cast::<HtmlInputElement>() {
            link_input.value()
        } else {
            return Err(());
        };

        let mut id = None;
        let mut max_uses = None;
        let mut valid_for = None;

        if let Some(advanced_mode) = refs.advanced_mode.cast::<HtmlInputElement>() {
            if advanced_mode.checked() {
                id = LinkConfig::parse_id(refs).unwrap();
                max_uses = LinkConfig::parse_max_uses(refs).unwrap();

                if let Some(expiration_input) = refs.expiration_input.cast::<HtmlInputElement>() {
                    if let Some(expiration_type_input) =
                        refs.expiration_type.cast::<HtmlInputElement>()
                    {
                        // TODO make this more concise
                        if ExpirationType::from(ToggleInputState::from(expiration_type_input.checked()))
                            == ExpirationType::Date
                        {
                            valid_for = LinkConfig::parse_date(refs).unwrap();
                        } else {
                            valid_for = LinkConfig::parse_duration(refs).unwrap();
                        }
                    }
                }
            }
        }

        Ok(Self {
            link,
            id,
            max_uses,
            valid_for,
        })
    }
}
