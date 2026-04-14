use napi::bindgen_prelude::ToNapiValue;

pub trait Arbitrary {
    type Output: ToNapiValue;
    fn generate(&self) -> Self::Output;
}
