use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_json::json;

#[serde_inline_default]
#[derive(Deserialize)]
struct Basic {
    // if using `String` you have to call `.to_string()`
    #[serde_inline_default("0.0.0.0".to_string())]
    host: String,
    // works without specifying the integer type at the end of the value (8080u16)
    #[serde_inline_default(8080)]
    port: u16,
    // expressions are working too
    #[serde_inline_default(serde_json::json!({}))]
    random_third_party_type: serde_json::Value,
}

fn main() -> Result<(), serde_json::Error> {
    // creating a empty json object to use the default value of all fields
    let json_object = json!({});
    let basic: Basic = serde_json::from_value(json_object)?;

    assert_eq!(basic.host, "0.0.0.0".to_string());
    assert_eq!(basic.port, 8080);
    assert_eq!(basic.random_third_party_type, json!({}));

    Ok(())
}
