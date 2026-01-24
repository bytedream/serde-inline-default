use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_json::json;

#[test]
fn enum_default() {
    #[serde_inline_default]
    #[derive(Debug, PartialEq, Eq, Deserialize)]
    #[serde(untagged)]
    enum Test {
        VariantWithFields {
            #[serde_inline_default(255)]
            test_int: u8,
        },
    }

    let enum_test: Test = serde_json::from_value(json!({"VariantWithFields": {}})).unwrap();

    assert_eq!(enum_test, Test::VariantWithFields { test_int: 255 })
}
