use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
enum JsonValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, JsonValue>),
    Null,
}

impl JsonValue {
    fn parse(input: &str) -> Result<JsonValue, String> {
        let input = input.trim();

        match input.chars().next() {
            Some('"') => Self::parse_string(input),
            Some('{') => Self::parse_object(&input[1..input.len() - 1]),
            Some(_) if input == "true" || input == "false" => Self::parse_boolean(input),
            Some(_) if input == "null" => Ok(JsonValue::Null),
            Some(_) => Self::parse_number(input),
            None => Err("Empty input".to_string()),
        }
    }

    fn parse_string(input: &str) -> Result<JsonValue, String> {
        if input.starts_with('"') && input.ends_with('"') {
            Ok(JsonValue::String(input[1..input.len() - 1].to_string()))
        } else {
            Err("Invalid string format".to_string())
        }
    }

    fn parse_number(input: &str) -> Result<JsonValue, String> {
        input
            .parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|_| "Invalid number format".to_string())
    }

    fn parse_boolean(input: &str) -> Result<JsonValue, String> {
        match input {
            "true" => Ok(JsonValue::Boolean(true)),
            "false" => Ok(JsonValue::Boolean(false)),
            _ => Err("Invalid boolean format".to_string()),
        }
    }

    fn parse_object(input: &str) -> Result<JsonValue, String> {
        let mut map = HashMap::new();
        let pairs = input
            .split(',')
            .map(|pair| pair.splitn(2, ':').map(str::trim).collect::<Vec<&str>>());

        for pair in pairs {
            if let [key, value] = pair.as_slice() {
                let key = key.trim_matches('"');
                let value = JsonValue::parse(value)?;
                map.insert(key.to_string(), value);
            } else {
                return Err("Invalid object entry".to_string());
            }
        }

        Ok(JsonValue::Object(map))
    }
}

// Implementing the `ToString` trait for `JsonValue` to allow stringification.
impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
            JsonValue::Object(map) => {
                let mut entries: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v))
                    .collect();
                entries.sort(); // Sort the entries by key to ensure consistent ordering
                write!(f, "{{{}}}", entries.join(","))
            }
        }
    }
}

fn main() {
    let json_str = r#"
    {
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "courses": null
    }
    "#;

    match JsonValue::parse(json_str) {
        Ok(parsed_json) => {
            println!("Parsed JSON: {:#?}", parsed_json);
            let json_string = parsed_json.to_string();
            println!("Stringified JSON: {}", json_string);
        }
        Err(e) => println!("Failed to parse JSON: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(
            JsonValue::parse("\"Hello\"").unwrap(),
            JsonValue::String("Hello".to_string())
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(JsonValue::parse("42").unwrap(), JsonValue::Number(42.0));
        assert!(JsonValue::parse("42.abc").is_err());
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(JsonValue::parse("true").unwrap(), JsonValue::Boolean(true));
        assert_eq!(
            JsonValue::parse("false").unwrap(),
            JsonValue::Boolean(false)
        );
        assert!(JsonValue::parse("tru").is_err());
    }

    #[test]
    fn test_parse_null() {
        assert_eq!(JsonValue::parse("null").unwrap(), JsonValue::Null);
    }

    #[test]
    fn test_parse_object() {
        let json_str = r#"
        {
            "key1": "value1",
            "key2": 10,
            "key3": false
        }
        "#;

        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), JsonValue::String("value1".to_string()));
        expected.insert("key2".to_string(), JsonValue::Number(10.0));
        expected.insert("key3".to_string(), JsonValue::Boolean(false));

        assert_eq!(
            JsonValue::parse(json_str).unwrap(),
            JsonValue::Object(expected)
        );
    }

    #[test]
    fn test_stringify() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), JsonValue::String("value1".to_string()));
        map.insert("key2".to_string(), JsonValue::Number(42.0));
        map.insert("key3".to_string(), JsonValue::Boolean(true));
        map.insert("key4".to_string(), JsonValue::Null);

        let json = JsonValue::Object(map);
        let json_string = json.to_string();

        assert_eq!(
            json_string,
            r#"{"key1":"value1","key2":42,"key3":true,"key4":null}"#
        );
    }
}
