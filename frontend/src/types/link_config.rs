use nonempty_collections::{nev, NEVec};
use serde::Serialize;
use time::{
    format_description::well_known::Iso8601,
    macros::time,
    Date,
    OffsetDateTime,
    UtcOffset,
};
use validated::{
    Validated,
    Validated::{Fail, Good},
};
use web_sys::HtmlInputElement;

use crate::{
    components::{expiration_input::ExpirationType, link_form::LinkFormRefs},
    types::{
        duration::{Duration, Parts},
        error::FormError,
    },
    util::server_config,
};

#[derive(Debug, Serialize, Clone)]
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

impl LinkConfig {
    // TODO incorporate server config errors
    // TODO when https://github.com/flamion/shorty/issues/51 is resolved apply the solution
    pub fn try_from(refs: &LinkFormRefs) -> Validated<Self, FormError> {
        let input = refs
            .advanced_mode
            .cast::<HtmlInputElement>()
            .expect(&format!(
                "Expected {:?} to be an HtmlInputElement",
                refs.advanced_mode
            ));

        let mut errors = vec![];

        let link = Self::parse_link(refs)
            .ok()
            .map_err(|e| errors.extend(e.into_iter()));

        let mut id = Ok(None);
        let mut max_uses = Ok(None);
        let mut valid_for = Ok(None);

        if input.checked() {
            id = Self::parse_id(refs)
                .ok()
                .map_err(|e| errors.extend(e.into_iter()));
            max_uses = Self::parse_max_uses(refs)
                .ok()
                .map_err(|e| errors.extend(e.into_iter()));
            valid_for = Self::parse_valid_for(refs)
                .ok()
                .map_err(|e| errors.extend(e.into_iter()));
        }

        if errors.is_empty() {
            Good(Self {
                link: link.unwrap(),
                id: id.unwrap(),
                max_uses: max_uses.unwrap(),
                valid_for: valid_for.unwrap(),
            })
        } else {
            Fail(NEVec::from_vec(errors).unwrap())
        }
    }

    fn parse_link(refs: &LinkFormRefs) -> Validated<String, FormError> {
        let input = refs.link_input.cast::<HtmlInputElement>().expect(&format!(
            "Expected {:?} to be an HtmlInputElement",
            refs.link_input
        ));

        let value = input.value();

        let mut errors = vec![];

        if let Some(config) = server_config() {
            if value.len() > config.max_link_length {
                errors.push(FormError::ExceededMaxLinkLength {
                    link: value.clone(),
                    max_length: config.max_link_length,
                });
            }
        }

        if value.is_empty() {
            errors.push(FormError::LinkInputEmpty);
        }

        if !errors.is_empty() {
            return Fail(NEVec::from_vec(errors).unwrap());
        }

        Good(value)
    }

    fn parse_id(refs: &LinkFormRefs) -> Validated<Option<String>, FormError> {
        let input = refs
            .custom_id_input
            .cast::<HtmlInputElement>()
            .expect(&format!(
                "Expected {:?} to be an HtmlInputElement",
                refs.custom_id_input
            ));

        let value = input.value();

        if value.is_empty() {
            return Good(None);
        }

        if let Some(config) = server_config() {
            if value.len() > config.max_custom_id_length {
                return Validated::fail(FormError::ExceededMaxIdLength {
                    id: value,
                    max_length: config.max_custom_id_length
                });
            }
        }

        Good(Some(value))
    }

    fn parse_max_uses(refs: &LinkFormRefs) -> Validated<Option<i64>, FormError> {
        let input = refs
            .max_usage_input
            .cast::<HtmlInputElement>()
            .expect(&format!(
                "Expected {:?} to be an HtmlInputElement",
                refs.max_usage_input
            ));

        let untrimmed = input.value();
        let value = untrimmed.trim();

        if value.is_empty() {
            return Good(None);
        }

        // TODO seems to always succeed because firefox does not set the value field if its not a number on type="number"
        // TODO https://bugzilla.mozilla.org/show_bug.cgi?id=1398528
        // TODO use input.validity.valid for mor fine grained errors https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.ValidityState.html
        let Ok(value) = value.parse::<i64>() else {
            return Validated::fail(FormError::ParseNumberFailure {
                number: value.to_string()
            });
        };

        if value < 0 {
            return Validated::fail(FormError::NegativeMaxUses { max_uses: value });
        }

        Good(Some(value))
    }

    fn parse_valid_for(refs: &LinkFormRefs) -> Validated<Option<i64>, FormError> {
        let input = refs
            .expiration_type
            .cast::<HtmlInputElement>()
            .expect(&format!(
                "Expected {:?} to be an HtmlInputElement",
                refs.expiration_type
            ));

        if ExpirationType::Date == ExpirationType::from(input.checked()) {
            Self::parse_date(refs)
        } else {
            Self::parse_duration(refs)
        }
    }

    fn parse_duration(refs: &LinkFormRefs) -> Validated<Option<i64>, FormError> {
        let input = refs
            .expiration_input
            .cast::<HtmlInputElement>()
            .expect(&format!(
                "Expected {:?} to be an HtmlInputElement",
                refs.expiration_input
            ));

        let value = input.value();

        if value == format!("{}", Duration::ZERO) {
            return Good(None);
        }

        let parts =
            Parts::try_from(value.as_str()).expect(&format!("Format unexpected: {}", value));

        let seconds = Duration::from_parts(parts).seconds * 1000;

        if seconds < 0 {
            return Validated::fail(FormError::NegativeExpiration { seconds });
        }

        Good(Some(seconds))
    }

    fn parse_date(refs: &LinkFormRefs) -> Validated<Option<i64>, FormError> {
        let input = refs
            .expiration_input
            .cast::<HtmlInputElement>()
            .expect(&format!(
                "Expected {:?} to be an HtmlInputElement",
                refs.expiration_input
            ));

        let value = input.value();

        if value.is_empty() {
            return Good(None);
        }

        let date = Date::parse(&input.value(), &Iso8601::DATE)
            .expect(&format!("Unexpected date format: {}", value));

        let date_time = date.with_time(time!(00:00)).assume_offset(UtcOffset::UTC);

        let local_now = OffsetDateTime::now_utc();

        let mut difference = date_time.unix_timestamp() - local_now.unix_timestamp();
        // because the timestamp is needed in milliseconds
        difference *= 1000;

        if difference < 0 {
            return Validated::fail(FormError::NegativeExpiration { seconds: difference });
        }

        Good(Some(difference))
    }
}
