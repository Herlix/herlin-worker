use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Body for the Request & Response
///
/// TODO: Being able to carry other than text
#[derive(Debug, Eq, PartialEq)]
pub enum Body {
    /// No value is set
    None,
    /// The body is empty
    Empty,
    /// Body as string
    Message(String),
}

impl Serialize for Body {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            Body::Message(v) => serializer.serialize_str(v),
            _ => serializer.serialize_str(""),
        }
    }
}

impl<'de> Deserialize<'de> for Body {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|i: Option<String>| match i {
            None => Body::None,
            Some(v) => {
                if v.is_empty() {
                    Body::Empty
                } else {
                    Body::Message(v)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_none() {
        let none = Body::None;
        assert_eq!(serde_json::to_string(&none).unwrap(), "\"\"".to_string());
    }

    #[test]
    fn serialize_empty() {
        let empty = Body::Empty;
        assert_eq!(serde_json::to_string(&empty).unwrap(), "\"\"".to_string());
    }

    #[test]
    fn serialize_message() {
        let message = Body::Message("something".to_string());
        assert_eq!(
            serde_json::to_string(&message).unwrap(),
            "\"something\"".to_string()
        );
    }

    #[test]
    fn deserialize_empty() {
        let empty: Body = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(empty, Body::Empty);
    }

    #[test]
    fn deserialize_message() {
        let message: Body = serde_json::from_str(r#""something""#).unwrap();
        assert_eq!(message, Body::Message("something".to_string()))
    }
}
