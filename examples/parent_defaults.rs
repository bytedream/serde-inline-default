use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use serde_json::json;

#[serde_inline_default]
#[derive(Serialize, Deserialize)]
struct A {
    /// Default value will be set from [A]'s default implementation,
    /// like this: `A::default().value`
    #[serde_inline_default_parent()]
    value: String,
}

const DEFAULT_STRING_VALUE: &str = "my cool default";
impl Default for A {
    fn default() -> Self {
        Self {
            value: DEFAULT_STRING_VALUE.into(),
        }
    }
}

fn main() -> Result<(), serde_json::Error> {
    let json_object = json!({});
    let basic: A = serde_json::from_value(json_object)?;
    assert_eq!(basic.value, DEFAULT_STRING_VALUE);
    Ok(())
}
