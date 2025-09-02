use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_json::json;
use std::borrow::Cow;

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

#[test]
fn test_lifetime() {
    #[serde_inline_default]
    #[derive(Deserialize)]
    struct LifetimeTest<'a> {
        #[serde_inline_default("test".into())]
        test_str: Cow<'a, str>,
    }

    let lifetime_test: LifetimeTest = serde_json::from_value(json!({})).unwrap();

    assert_eq!(lifetime_test.test_str, "test");
}

#[test]
#[allow(dead_code)]
fn test_conditional_compilation() {
    #[cfg(debug_assertions)]
    #[derive(Deserialize)]
    struct TypeA(u8);

    #[cfg(not(debug_assertions))]
    #[derive(Deserialize)]
    struct TypeB(u8);

    #[serde_inline_default]
    #[derive(Deserialize)]
    struct Test {
        #[cfg(debug_assertions)]
        #[serde_inline_default(TypeA(1))]
        val_a: TypeA,
        #[cfg(not(debug_assertions))]
        #[serde_inline_default(TypeB(1))]
        val_b: TypeB,
    }
}
