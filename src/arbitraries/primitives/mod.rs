use napi::bindgen_prelude::{Either10, External};
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
    BooleanArbitrary,
    BigIntArbitrary,
    IntegerArbitrary,
    NatArbitrary,
    MaxSafeIntegerArbitrary,
    MaxSafeNatArbitrary,
    StringArbitrary,
    FloatArbitrary,
    DoubleArbitrary,
    DateArbitrary<'a>,
>;
