use napi_derive::napi;

#[napi]
pub enum StringUnit {
    Ascii,
    Unicode,
    Grapheme,
    GraphemeComposite,
    GraphemeAscii,
    BinaryAscii,
}

#[napi(object)]
pub struct StringOption {
    pub unit: Option<StringUnit>,
    pub min_length: Option<i64>,
    pub max_length: Option<i64>,
    pub size: Option<i64>,
}

#[napi]
pub fn string(option: StringOption) -> napi::Result<String> {
    let unit = option.unit.unwrap_or(StringUnit::Unicode);
    let min_length = option.min_length.unwrap_or(0);
    let max_length = option.max_length.unwrap_or(100);

    let size = option.size.unwrap_or_else(|| {
        if min_length > max_length {
            min_length
        } else {
            rand::random::<i64>() % (max_length - min_length + 1) + min_length
        }
    });

    let chars: Vec<char> = match unit {
        StringUnit::Ascii => (0..size).map(|_| rand::random::<u8>() as char).collect(),
        StringUnit::Unicode => (0..size).map(|_| rand::random::<char>()).collect(),
        StringUnit::Grapheme => (0..size).map(|_| rand::random::<char>()).collect(),
        StringUnit::GraphemeComposite => (0..size).map(|_| rand::random::<char>()).collect(),
        StringUnit::GraphemeAscii => (0..size).map(|_| rand::random::<u8>() as char).collect(),
        StringUnit::BinaryAscii => (0..size).map(|_| rand::random::<u8>() as char).collect(),
    };

    Ok(chars.into_iter().collect())
}
