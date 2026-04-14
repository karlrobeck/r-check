use napi::bindgen_prelude::{BigInt, External};
use napi_derive::napi;

use crate::traits::Arbitrary;

pub struct BigIntOption {
    pub min: Option<BigInt>,
    pub max: Option<BigInt>,
}

#[napi]
pub struct BigIntArbitrary(pub(crate) Option<BigIntOption>);

impl Arbitrary for BigIntArbitrary {
    type Output = u64;

    fn generate(&self) -> Self::Output {
        let option = self.0.as_ref();
        let min = option
            .and_then(|o| o.min.as_ref())
            .map(|b| b.get_u64().1)
            .unwrap_or(0);
        let max = option
            .and_then(|o| o.max.as_ref())
            .map(|b| b.get_u64().1)
            .unwrap_or(u64::MAX);

        rand::random::<u64>() % (max - min + 1) + min
    }
}

#[napi]
pub fn bigint(min: Option<BigInt>, max: Option<BigInt>) -> External<BigIntArbitrary> {
    External::new(BigIntArbitrary(Some(BigIntOption { min, max })))
}
