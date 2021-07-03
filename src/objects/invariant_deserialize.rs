//! The goal of this is to deserialize from json strange struct members that can be actual numbers (e.g. id = 777) in some cases and
//! strings in other cases (i.g. id = "777"). The Json value type varies depending on parent struct which is being deserialized.
//!
//! An example. The id in photo::Album is Integer. The NewsAttachment::album which is the same as photo::Album differs only in id that is the String
//! (at least in Feb'2021, later it became an Integer being the braking change).
//!
//! Another example, the Link::product::price::amount might be string in some json responses as well as number in others

use core::marker::PhantomData;
use num::cast::FromPrimitive;
use serde::{
    de::{Error as SerdeError, Unexpected, Visitor},
    Deserializer,
};
use std::{any::type_name, fmt, str::FromStr};

pub struct ToNum<T> {
    _phantom: PhantomData<T>,
}

impl<T: FromStr + FromPrimitive> ToNum<T> {
    pub fn deserialize<'de, D>(de: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(ToNum::<T>::new())
    }

    pub fn deserialize_opt<'de, D>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(ToNum::<T>::new())
            .map_or(Ok(None), |v| Ok(Some(v)))
    }

    pub fn new() -> Self {
        Self {
            _phantom: PhantomData {},
        }
    }
}

impl<'de, T: FromStr + FromPrimitive> Visitor<'de> for ToNum<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            r#"a string representing legal {} value or a value of {} itself"#,
            type_name::<T>(),
            type_name::<T>()
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        if let Ok(v) = s.parse::<T>() {
            Ok(v)
        } else {
            Err(SerdeError::invalid_value(Unexpected::Str(s), &self))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: SerdeError,
        T: FromPrimitive,
    {
        T::from_u64(v).ok_or(SerdeError::invalid_value(Unexpected::Unsigned(v), &self))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: SerdeError,
        T: FromPrimitive,
    {
        T::from_i64(v).ok_or(SerdeError::invalid_value(Unexpected::Signed(v), &self))
    }
}

#[cfg(test)]
mod test_i64 {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Item {
        #[serde(deserialize_with = "ToNum::<i64>::deserialize")]
        value: i64,
    }

    #[test]
    fn i64_deserialize_positive_i64() {
        let json = r#"
        {
            "value": 123
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, 123);
    }

    #[test]
    fn i64_deserialize_positive_string() {
        let json = r#"
        {
            "value": "123"
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, 123);
    }

    #[test]
    fn i64_deserialize_negative_i64() {
        let json_item = r#"
        {
            "value": -123
        }
        "#;
        let item = serde_json::from_str::<Item>(json_item).unwrap();
        assert_eq!(item.value, -123);
    }

    #[test]
    fn i64_deserialize_negative_string() {
        let json_item = r#"
        {
            "value": "-123"
        }
        "#;
        let item = serde_json::from_str::<Item>(json_item).unwrap();
        assert_eq!(item.value, -123);
    }

    #[test]
    fn i64_dont_deserialize_too_large_u64() {
        let json_item = format!(r#"{{"value": "{}"\}}"#, std::i64::MAX as u64 + 1);
        let res = serde_json::from_str::<Item>(&json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i64_dont_deserialize_positive_float_value() {
        let json_item = r#"
        {
            "value": 123.0
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i64_dont_deserialize_positive_float_string() {
        let json_item = r#"
        {
            "value": "123.0"
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i64_dont_deserialize_negative_float_value() {
        let json_item = r#"
        {
            "value": -123.0
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i64_dont_deserialize_negative_float_string() {
        let json_item = r#"
        {
            "value": "-123.0"
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i64_dont_deserialize_arbitrary_string() {
        let json_item = r#"
        {
            "id": "-+"
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod test_i16 {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Item {
        #[serde(deserialize_with = "ToNum::<i16>::deserialize")]
        value: i16,
    }

    #[test]
    fn i16_deserialize_positive_i64() {
        let json = r#"
        {
            "value": 123
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, 123);
    }

    #[test]
    fn i16_deserialize_positive_string() {
        let json = r#"
        {
            "value": "123"
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, 123);
    }

    #[test]
    fn i16_deserialize_negative_i64() {
        let json_item = r#"
        {
            "value": -123
        }
        "#;
        let item = serde_json::from_str::<Item>(json_item).unwrap();
        assert_eq!(item.value, -123);
    }

    #[test]
    fn i16_deserialize_negative_string() {
        let json_item = r#"
        {
            "value": "-123"
        }
        "#;
        let item = serde_json::from_str::<Item>(json_item).unwrap();
        assert_eq!(item.value, -123);
    }

    #[test]
    fn i16_dont_deserialize_too_large_u16() {
        let json_item = format!(r#"{{"value": "{}"\}}"#, std::i16::MAX as u16 + 1);
        let res = serde_json::from_str::<Item>(&json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i16_dont_deserialize_positive_float_value() {
        let json_item = r#"
        {
            "value": 123.0
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i16_dont_deserialize_positive_float_string() {
        let json_item = r#"
        {
            "value": "123.0"
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i16_dont_deserialize_negative_float_value() {
        let json_item = r#"
        {
            "value": -123.0
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i16_dont_deserialize_negative_float_string() {
        let json_item = r#"
        {
            "value": "-123.0"
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }

    #[test]
    fn i16_dont_deserialize_arbitrary_string() {
        let json_item = r#"
        {
            "id": "-+"
        }
        "#;
        let res = serde_json::from_str::<Item>(json_item);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod test_i64_opt {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Default)]
    struct Item {
        #[serde(default)]
        #[serde(deserialize_with = "ToNum::<i64>::deserialize_opt")]
        value: Option<i64>,
    }

    #[test]
    fn i64_deserialize_positive_i64() {
        let json = r#"
        {
            "value": 123
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, Some(123));
    }

    #[test]
    fn i64_deserialize_none() {
        let json = r#"
        {
            "no_value": 123
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, None);
    }
}

pub struct ToStr;

impl<'de> Visitor<'de> for ToStr {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            r#"a string or a value of type that implements ToString"#
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(s.to_owned())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(format!("{}", v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(format!("{}", v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(format!("{}", v))
    }
}

impl ToStr {
    pub fn deserialize<'de, D>(de: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(ToStr {})
    }

    pub fn deserialize_opt<'de, D>(de: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(ToStr {})
            .map_or(Ok(None), |s| Ok(Some(s)))
    }
}

#[cfg(test)]
mod test_str {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Item {
        #[serde(deserialize_with = "ToStr::deserialize")]
        value: String,
    }

    #[test]
    fn str_deserialize_positive_i64() {
        let json = r#"
        {
            "value": 123
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, "123");
    }

    #[test]
    fn str_deserialize_negative_i64() {
        let json = r#"
        {
            "value": -123
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, "-123");
    }

    #[test]
    fn str_deserialize_string() {
        let json = r#"
        {
            "value": "123"
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, "123");
    }

    #[test]
    fn str_deserialize_positive_f64() {
        let json = r#"
        {
            "value": 123.4
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, "123.4");
    }

    #[test]
    fn str_deserialize_negative_f64() {
        let json = r#"
        {
            "value": -123.4
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, "-123.4");
    }
}

#[cfg(test)]
mod test_str_opt {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Default)]
    struct Item {
        #[serde(default)]
        #[serde(deserialize_with = "ToStr::deserialize_opt")]
        value: Option<String>,
    }

    #[test]
    fn str_deserialize_negative_f64() {
        let json = r#"
        {
            "value": -123.4
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, Some("-123.4".to_string()));
    }

    #[test]
    fn str_deserialize_none() {
        let json = r#"
        {
            "no_value": -123.4
        }
        "#;
        let item = serde_json::from_str::<Item>(json).unwrap();
        assert_eq!(item.value, None);
    }
}
