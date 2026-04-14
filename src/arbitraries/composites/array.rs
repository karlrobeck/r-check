use napi::bindgen_prelude::{Either10, External};
use napi_derive::napi;

use crate::{
    arbitraries::primitives::{PrimitiveArbs, PrimitiveValues},
    traits::Arbitrary,
};

#[napi]
pub struct TupleArbitrary(pub(crate) Vec<PrimitiveArbs<'static>>);

impl Arbitrary for TupleArbitrary {
    type Output = Vec<PrimitiveValues>;

    fn generate(&self) -> Self::Output {
        let arr = &self.0;
        arr.iter()
            .map(|item| match item {
                Either10::A(bool) => Either10::A(bool.generate()),
                Either10::B(big_int) => Either10::B(big_int.generate()),
                Either10::C(integer) => Either10::C(integer.generate()),
                Either10::D(nat) => Either10::D(nat.generate()),
                Either10::E(max_safe_integer) => Either10::E(max_safe_integer.generate()),
                Either10::F(max_safe_nat) => Either10::F(max_safe_nat.generate()),
                Either10::G(string) => Either10::G(string.generate()),
                Either10::H(float) => Either10::H(float.generate()),
                Either10::I(double) => Either10::I(double.generate()),
                Either10::J(date) => Either10::J(date.generate()),
            })
            .collect()
    }
}

#[napi]
pub fn tuple<'a>(arbs: Vec<PrimitiveArbs<'a>>) -> napi::Result<External<TupleArbitrary>> {
    Ok(External::new(TupleArbitrary(arbs)))
}
