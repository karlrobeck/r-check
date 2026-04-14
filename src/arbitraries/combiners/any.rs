use napi::{Either, bindgen_prelude::External};
use napi_derive::napi;

use crate::{
    arbitraries::primitives::{PrimitiveArbs, PrimitiveValues},
    traits::Arbitrary,
};

#[napi(object)]
pub struct OneOfItemOptions {
    pub arbitrary: PrimitiveArbs<'static>,
    pub weight: Option<u32>,
}

#[napi(object)]
pub struct OneOfOptions {
    pub with_cross_shrink: Option<bool>,
    pub max_depth: Option<u32>,
    pub depth_size: Option<u32>,
    pub depth_identifier: Option<String>,
}

#[napi]
pub struct OneOfArbitrary {
    options: Option<OneOfOptions>,
    items: Vec<OneOfItemOptions>,
}

impl Arbitrary for OneOfArbitrary {
    type Output = PrimitiveValues;

    fn generate(&self) -> Self::Output {
        todo!("")
    }
}

#[napi]
pub fn oneof(
    arbs: Either<Vec<OneOfItemOptions>, Vec<PrimitiveArbs>>,
    options: Option<OneOfOptions>,
) -> napi::Result<External<OneOfArbitrary>> {
    let arbs = match arbs {
        Either::A(vec) => vec,
        Either::B(arb) => arb
            .into_iter()
            .map(|a| OneOfItemOptions {
                arbitrary: a,
                weight: None,
            })
            .collect(),
    };

    Ok(External::new(OneOfArbitrary {
        options,
        items: arbs,
    }))
}
