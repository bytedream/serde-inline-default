use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_json::json;

#[test]
fn test_serde_inline_default() {
    fn native_default() -> u32 {
        69
    }

    #[serde_inline_default]
    #[derive(Deserialize)]
    struct Test {
        #[serde(default = "native_default")]
        native: u32,
        #[serde_inline_default(420)]
        inline: u32,
        #[serde_inline_default(-1337)]
        inline_negative: i32,
        #[serde_inline_default("string".to_string())]
        string: String,
    }

    let test: Test = serde_json::from_value(json!({})).unwrap();

    assert_eq!(test.native, 69);
    assert_eq!(test.inline, 420);
    assert_eq!(test.inline_negative, -1337);
    assert_eq!(test.string, "string".to_string());
}
