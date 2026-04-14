use napi::bindgen_prelude::{Either10, External, ExternalRef};
use napi_derive::napi;

use crate::arbitraries::primitives::{
    big_int::BigIntArbitrary,
    boolean::BooleanArbitrary,
    date::DateArbitrary,
    number::{
        DoubleArbitrary, FloatArbitrary, IntegerArbitrary, MaxSafeIntegerArbitrary,
        MaxSafeNatArbitrary, NatArbitrary,
    },
    string::StringArbitrary,
};

pub mod big_int;
pub mod boolean;
pub mod date;
pub mod number;
pub mod string;

#[napi]
pub type PrimitiveArbs<'a> = Either10<
    ExternalRef<BooleanArbitrary>,
    ExternalRef<BigIntArbitrary>,
    ExternalRef<IntegerArbitrary>,
    ExternalRef<NatArbitrary>,
    ExternalRef<MaxSafeIntegerArbitrary>,
    ExternalRef<MaxSafeNatArbitrary>,
    ExternalRef<StringArbitrary>,
    ExternalRef<FloatArbitrary>,
    ExternalRef<DoubleArbitrary>,
    ExternalRef<DateArbitrary<'a>>,
>;

#[napi]
pub type PrimitiveValues = Either10<bool, u128, i32, u32, i64, u64, String, f32, f64, f64>;
