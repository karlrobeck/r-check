use napi::{Env, bindgen_prelude::Reference};
use napi_derive::napi;

use crate::{arbitraries::primitives::PrimitiveArbs, traits::Arbitrary};

#[napi]
pub struct TupleArbitrary(pub(crate) Vec<Reference<PrimitiveArbs<'static>>>);

impl Arbitrary for TupleArbitrary {
    type Output = ();

    fn generate(&self) -> Self::Output {
        let arr = &self.0;
        // arr.iter().map(|item|)

        todo!("")
    }
}

#[napi]
pub fn tuple<'a>(
    env: Env,
    arbs: Vec<Reference<PrimitiveArbs<'a>>>,
) -> napi::Result<TupleArbitrary> {
    // let arbs = arbs.into();

    todo!("");
}
