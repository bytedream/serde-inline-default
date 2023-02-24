use serde_json::json;
macro_rules! simple_macro {
    (struct $name:ident { $($field:ident: $type:ty $(= $default:expr)?),*$(,)? }) => {
        #[serde_inline_default::serde_inline_default]
        #[derive(serde::Deserialize)]
        struct $name {
            $(
                $(
                    #[serde_inline_default($default)]
                )?
                $field: $type
            ),*
        }
    }
}

fn main() -> Result<(), serde_json::Error> {
    // `username` and `password` must be set when deserializing as no default value is defined for
    // them. `secret` not as we're defining a default value for it
    simple_macro! {
        struct Example {
            username: String,
            password: String,
            secret: String = "verysecretsecret".to_string()
        }
    }

    let json_object = json!({
        "username": "testuser",
        "password": "testpassword"
    });

    let example: Example = serde_json::from_value(json_object)?;
    assert_eq!(example.username, "testuser");
    assert_eq!(example.password, "testpassword");
    assert_eq!(example.secret, "verysecretsecret");

    Ok(())
}
