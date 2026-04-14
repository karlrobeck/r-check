use napi::{JsDate, bindgen_prelude::External};
use napi_derive::napi;

use crate::traits::Arbitrary;

#[napi(object)]
pub struct DateOption<'a> {
    pub min: Option<JsDate<'a>>,
    pub max: Option<JsDate<'a>>,
    pub no_invalid_date: Option<bool>,
}

#[napi]
pub struct DateArbitrary<'a>(pub(crate) DateOption<'a>);

impl<'a> Arbitrary for DateArbitrary<'a> {
    type Output = f64;

    fn generate(&self) -> Self::Output {
        let option = &self.0;

        let min = option
            .min
            .as_ref()
            .map(|d| d.value_of().unwrap_or(0.0) as u64)
            .unwrap_or(0);

        let max = option
            .max
            .as_ref()
            .map(|d| d.value_of().unwrap_or(0.0) as u64)
            .unwrap_or(u64::MAX);

        let no_invalid_date = option.no_invalid_date.unwrap_or(false);

        if no_invalid_date && min > max {
            // return std::time::UNIX_EPOCH + std::time::Duration::from_secs(min);
        }

        let timestamp = rand::random::<u64>() % (max - min + 1) + min;
        // std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp)

        (0.0)
    }
}

pub fn date<'a>(option: DateOption<'a>) -> napi::Result<External<DateArbitrary<'a>>> {
    Ok(External::new(DateArbitrary(option)))
}
