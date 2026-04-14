use napi_derive::napi;

#[napi]
pub fn boolean() -> napi::Result<bool> {
    Ok(rand::random())
}
