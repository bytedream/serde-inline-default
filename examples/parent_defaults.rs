use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Serialize, Deserialize)]
struct A {
   #[serde_inline_parent()]
   value: String,
}
impl Default for A {
   fn default() -> Self {
      Self {
         value: "my cool default".to_string(),
      }
   }
}


fn main() -> Result<(), serde_json::Error> {
   let json_object = json!({});
   let basic: A = serde_json::from_value(json_object)?;
   assert_eq!(basic.value, "my cool default".to_string());


   Ok(())
}
