use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use clap::Parser;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let reader: Box<dyn io::Read> = if args.file == "-" {
        Box::new(io::stdin())
    } else {
        let path = Path::new(&args.file);
        Box::new(File::open(path)?)
    };
    let reader = BufReader::new(reader);
    let value = serde_json::from_reader(reader)?;
    for value in FlatValueIterator::new(&value) {
        println!("{}", value);
    }
    Ok(())
}

/// Prints a JSON value as a flat list of key-value pairs.
///
/// Each line is a key-value pair, where the key is a JSON Pointer and the value is a JSON value.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// The JSON file to read
    #[arg(value_name = "FILE")]
    file: String,
}

struct FlatValueIterator<'a> {
    stack: Vec<(String, &'a Value)>,
}

impl<'a> FlatValueIterator<'a> {
    fn new(value: &'a Value) -> Self {
        FlatValueIterator {
            stack: vec![("$".to_string(), value)],
        }
    }
}

impl<'a> Iterator for FlatValueIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((parent, value)) = self.stack.pop() {
            match value {
                Value::Null => return Some(format!("{}: null", parent)),
                Value::Bool(bool) => return Some(format!("{}: {}", parent, bool)),
                Value::Number(number) => return Some(format!("{}: {}", parent, number)),
                Value::String(string) => return Some(format!("{}: {:?}", parent, string)),
                Value::Array(values) => {
                    for (index, value) in values.iter().enumerate().rev() {
                        let new_parent = format!("{}[{}]", parent, index);
                        self.stack.push((new_parent, value));
                    }
                }
                Value::Object(map) => {
                    for (name, value) in map.iter().rev() {
                        let new_parent = format!("{}.{}", parent, name);
                        self.stack.push((new_parent, value));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_value_iterator_full() {
        let value = serde_json::json!({
            "null": null,
            "bool": true,
            "number": 42,
            "string": "Hello, world!",
            "array": [1, 2, 3],
            "object": {
                "foo": "bar",
                "answer": 42
            }
        });

        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some("$.null: null".to_string()));
        assert_eq!(iter.next(), Some("$.bool: true".to_string()));
        assert_eq!(iter.next(), Some("$.number: 42".to_string()));
        assert_eq!(
            iter.next(),
            Some(r#"$.string: "Hello, world!""#.to_string())
        );
        assert_eq!(iter.next(), Some("$.array[0]: 1".to_string()));
        assert_eq!(iter.next(), Some("$.array[1]: 2".to_string()));
        assert_eq!(iter.next(), Some("$.array[2]: 3".to_string()));
        assert_eq!(iter.next(), Some(r#"$.object.foo: "bar""#.to_string()));
        assert_eq!(iter.next(), Some("$.object.answer: 42".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_flat_value_iterator_null() {
        let value = serde_json::json!(null);
        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some(r#"$: null"#.to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_flat_value_iterator_bool() {
        let value = serde_json::json!(true);
        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some(r#"$: true"#.to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_flat_value_iterator_number() {
        let value = serde_json::json!(42);
        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some(r#"$: 42"#.to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_flat_value_iterator_string() {
        let value = serde_json::json!("Hello, world!");
        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some(r#"$: "Hello, world!""#.to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_flat_value_iterator_object() {
        let value = serde_json::json!({
            "foo": "bar",
            "answer": 42
        });
        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some("$.foo: \"bar\"".to_string()));
        assert_eq!(iter.next(), Some("$.answer: 42".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_flat_value_iterator_object_nested() {
        let value = serde_json::json!({
            "foo": "bar",
            "answer": 42,
            "object": {
                "foo": "bar",
                "answer": 42
            }
        });
        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some("$.foo: \"bar\"".to_string()));
        assert_eq!(iter.next(), Some("$.answer: 42".to_string()));
        assert_eq!(iter.next(), Some("$.object.foo: \"bar\"".to_string()));
        assert_eq!(iter.next(), Some("$.object.answer: 42".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_flat_value_iterator_array() {
        let value = serde_json::json!([1, 2, 3]);
        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some("$[0]: 1".to_string()));
        assert_eq!(iter.next(), Some("$[1]: 2".to_string()));
        assert_eq!(iter.next(), Some("$[2]: 3".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_flat_value_iterator_array_nested() {
        let value = serde_json::json!([1, 2, 3, [4, 5, 6]]);
        let mut iter = FlatValueIterator::new(&value);
        assert_eq!(iter.next(), Some("$[0]: 1".to_string()));
        assert_eq!(iter.next(), Some("$[1]: 2".to_string()));
        assert_eq!(iter.next(), Some("$[2]: 3".to_string()));
        assert_eq!(iter.next(), Some("$[3][0]: 4".to_string()));
        assert_eq!(iter.next(), Some("$[3][1]: 5".to_string()));
        assert_eq!(iter.next(), Some("$[3][2]: 6".to_string()));
        assert_eq!(iter.next(), None);
    }
}
