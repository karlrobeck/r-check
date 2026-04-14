use napi::bindgen_prelude::External;
use napi_derive::napi;

use crate::traits::Arbitrary;

#[derive(Clone)]
#[napi]
pub struct BooleanArbitrary;

impl Arbitrary for BooleanArbitrary {
    type Output = bool;

    fn generate(&self) -> Self::Output {
        rand::random()
    }
}

#[napi]
pub fn boolean() -> napi::Result<External<BooleanArbitrary>> {
    Ok(External::new(BooleanArbitrary))
}
