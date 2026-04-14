use napi_derive::napi;

#[napi(object)]
pub struct IntegerOption {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[napi]
pub fn integer(option: IntegerOption) -> napi::Result<i32> {
    let min = option.min.unwrap_or(i32::MIN);
    let max = option.max.unwrap_or(i32::MAX);
    Ok(rand::random::<i32>() % (max - min + 1) + min)
}

#[napi(object)]
pub struct NatOption {
    pub max: Option<u32>,
}

#[napi]
pub fn nat(option: NatOption) -> napi::Result<u32> {
    let max = option.max.unwrap_or(u32::MAX);
    Ok(rand::random::<u32>() % (max + 1))
}

#[napi]
pub fn max_safe_integer() -> napi::Result<i64> {
    const MAX_SAFE_INTEGER: i64 = 9007199254740991; // 2^53 - 1
    Ok((rand::random::<u64>() % (MAX_SAFE_INTEGER as u64 + 1)) as i64)
}

#[napi]
pub fn max_safe_nat() -> napi::Result<u32> {
    const MAX_SAFE_INTEGER: u64 = 9007199254740991; // 2^53 - 1
    Ok((rand::random::<u64>() % (MAX_SAFE_INTEGER + 1)) as u32)
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
pub fn float(option: FloatOption) -> napi::Result<f32> {
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

    Ok(result)
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

pub fn double(option: DoubleOption) -> napi::Result<f64> {
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

    Ok(result)
}
