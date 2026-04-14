use napi::bindgen_prelude::External;
use napi_derive::napi;

use crate::traits::Arbitrary;

#[napi(object)]
pub struct IntegerOption {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[napi]
pub struct IntegerArbitrary(pub(crate) Option<IntegerOption>);

impl Arbitrary for IntegerArbitrary {
    type Output = i32;

    fn generate(&self) -> Self::Output {
        let option = self.0.as_ref();
        let min = option.and_then(|o| o.min).unwrap_or(i32::MIN);
        let max = option.and_then(|o| o.max).unwrap_or(i32::MAX);
        rand::random::<i32>() % (max - min + 1) + min
    }
}

#[napi]
pub fn integer(option: Option<IntegerOption>) -> napi::Result<External<IntegerArbitrary>> {
    Ok(External::new(IntegerArbitrary(option)))
}

#[napi(object)]
pub struct NatOption {
    pub max: Option<u32>,
}

#[napi]
pub struct NatArbitrary(pub(crate) Option<NatOption>);

impl Arbitrary for NatArbitrary {
    type Output = u32;

    fn generate(&self) -> Self::Output {
        let option = self.0.as_ref();
        let max = option.and_then(|o| o.max).unwrap_or(u32::MAX);
        rand::random::<u32>() % (max + 1)
    }
}

#[napi]
pub fn nat(option: Option<NatOption>) -> napi::Result<External<NatArbitrary>> {
    Ok(External::new(NatArbitrary(option)))
}

#[napi]
pub struct MaxSafeIntegerArbitrary;

impl Arbitrary for MaxSafeIntegerArbitrary {
    type Output = i64;

    fn generate(&self) -> Self::Output {
        const MAX_SAFE_INTEGER: i64 = 9007199254740991; // 2^53 - 1
        (rand::random::<u64>() % (MAX_SAFE_INTEGER as u64 + 1)) as i64
    }
}

#[napi]
pub fn max_safe_integer() -> napi::Result<External<MaxSafeIntegerArbitrary>> {
    Ok(External::new(MaxSafeIntegerArbitrary))
}

#[napi]
pub struct MaxSafeNatArbitrary;

impl Arbitrary for MaxSafeNatArbitrary {
    type Output = u32;

    fn generate(&self) -> Self::Output {
        const MAX_SAFE_INTEGER: u64 = 9007199254740991; // 2^53 - 1
        (rand::random::<u64>() % (MAX_SAFE_INTEGER + 1)) as u32
    }
}

#[napi]
pub fn max_safe_nat() -> napi::Result<External<MaxSafeNatArbitrary>> {
    Ok(External::new(MaxSafeNatArbitrary))
}

#[napi(object)]
pub struct FloatOption {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_excluded: Option<bool>,
    pub max_excluded: Option<bool>,
    pub no_default_infinity: Option<bool>,
    pub no_default_nan: Option<bool>,
    pub no_integer: Option<bool>,
}

#[napi]
pub struct FloatArbitrary(pub(crate) FloatOption);

impl Arbitrary for FloatArbitrary {
    type Output = f32;

    fn generate(&self) -> Self::Output {
        let option = &self.0;
        let min = option.min.map(|v| v as f32).unwrap_or(f32::MIN);
        let max = option.max.map(|v| v as f32).unwrap_or(f32::MAX);
        let mut result = rand::random::<f32>() * (max - min) + min;

        if option.no_default_infinity.unwrap_or(false) {
            while result.is_infinite() {
                result = rand::random::<f32>() * (max - min) + min;
            }
        }

        if option.no_default_nan.unwrap_or(false) {
            while result.is_nan() {
                result = rand::random::<f32>() * (max - min) + min;
            }
        }

        if option.no_integer.unwrap_or(false) {
            while result.fract() == 0.0 {
                result = rand::random::<f32>() * (max - min) + min;
            }
        }

        result
    }
}

#[napi]
pub fn float(option: FloatOption) -> napi::Result<External<FloatArbitrary>> {
    Ok(External::new(FloatArbitrary(option)))
}

#[napi(object)]
pub struct DoubleOption {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_excluded: Option<bool>,
    pub max_excluded: Option<bool>,
    pub no_default_infinity: Option<bool>,
    pub no_default_nan: Option<bool>,
    pub no_integer: Option<bool>,
}

#[napi]
pub struct DoubleArbitrary(pub(crate) DoubleOption);

impl Arbitrary for DoubleArbitrary {
    type Output = f64;

    fn generate(&self) -> Self::Output {
        let option = &self.0;
        let min = option.min.unwrap_or(f64::MIN);
        let max = option.max.unwrap_or(f64::MAX);
        let mut result = rand::random::<f64>() * (max - min) + min;

        if option.no_default_infinity.unwrap_or(false) {
            while result.is_infinite() {
                result = rand::random::<f64>() * (max - min) + min;
            }
        }

        if option.no_default_nan.unwrap_or(false) {
            while result.is_nan() {
                result = rand::random::<f64>() * (max - min) + min;
            }
        }

        if option.no_integer.unwrap_or(false) {
            while result.fract() == 0.0 {
                result = rand::random::<f64>() * (max - min) + min;
            }
        }

        result
    }
}

#[napi]
pub fn double(option: DoubleOption) -> napi::Result<External<DoubleArbitrary>> {
    Ok(External::new(DoubleArbitrary(option)))
}
