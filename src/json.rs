use std::fs::{OpenOptions, create_dir, read_to_string};
use std::io::prelude::*;
use std::io::{ErrorKind, Error};
use std::collections::HashMap;

use super::network::Network;
use super::component::*;
use super::node::Node;

/// Strip the first and last character of a string
fn strip_outer_characters(s: &str) -> String {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

/// Trait to (de)serialize objects to/from json
pub trait JsonSerializable: JsonReadable + JsonWritable {
}

pub trait JsonReadable {
    /// Get object intance from json representation
    fn from_json(filename: &str) -> Result<Box<Self>, String>;
}

pub trait JsonWritable {
    /// Get json representation of object
    fn to_json(&self) -> Result<(), String>;
}

struct JsonReader<'a> {
    filename: &'a str,
    keys: HashMap<String, String>,
}

impl JsonReader<'_> {
    fn new(filename: &str) -> JsonReader {
        JsonReader {
            filename,
            keys: HashMap::new(),
        }
    }

    fn get_key(&self, key: &str) -> Result<&str, String> {
        match self.keys.get(key) {
            Some(s) => Ok(s),
            None => Err(format!("Invalid Json: Key {} not found in json file {}", key, self.filename)),
        }
    }

    fn read(&mut self) -> Result<(), String> {
        let mut json: String;
        match read_to_string(format!("{}/{}.json", "json", self.filename)) {
            Err(e) => panic!("{}", e),
            Ok(s) => {
                json = s;
                json.retain(|c| !c.is_whitespace());
                json = strip_outer_characters(json.as_str()); // Strip braces
                let split = json.split(',');
                for item in split.clone() {
                    let split2 = item.split(':').collect::<Vec<&str>>();
                    let key = split2.get(0);
                    let value = split2.get(1);
                    match (key, value) {
                        (Some(k), Some(v)) => {
                            let k = strip_outer_characters(k); // Strip quotes
                            let v = strip_outer_characters(v); // Strip quotes
                            self.keys.insert(k, v);
                        },
                        (_, _) => return Err(format!("Failed to read json file {} on k-v pair {}", self.filename, item)),
                    }
                }
                Ok(())
            }
        }
    }
}

struct JsonWriter<'a> {
    filename: &'a str,
    json: String,
}

impl JsonWriter<'_> {
    fn new(filename: &str) -> JsonWriter {
        match create_dir("json") {
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {},
            Err(e) => panic!("{}", e),
            Ok(_) => {},
        }
        JsonWriter {
            filename,
            json: "{\n".to_string(),
        }
    }

    fn add_key<T: ToString>(&mut self, key: &'static str, value: T) {
        let s = format!("    \"{}\": \"{}\",\n", key, value.to_string());
        self.json += s.as_str();
    }

    fn write(&mut self) -> std::io::Result<()> {
        let mut chars = self.json.chars();
        chars.next_back();
        chars.next_back();
        self.json = chars.as_str().to_string() + "\n}";

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}/{}.json", "json", self.filename))?;

        file.write_all(self.json.as_bytes())?;
        Ok(())
    }
}

impl JsonWritable for dyn Component {
    fn to_json(&self) -> Result<(), String> {
        let mut writer = JsonWriter::new(self.name().as_str());
        writer.add_key("name", self.name());
        writer.add_key("type", format!("{}", self.r#type()));
        match writer.write() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}

impl JsonReadable for CircuitBreaker {
    fn from_json(filename: &str) -> Result<Box<CircuitBreaker>, String> {
        let mut reader = JsonReader::new(filename);
        reader.read()?;
        let name = reader.get_key("name")?;
        let r#type = reader.get_key("type")?;
        if r#type == "CircuitBreaker" {
            Ok(Box::new(CircuitBreaker::new(name)))
        } else {
            Err(format!("Invalid json: component type CircuitBreaker expected in file {}", filename))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_invalid_files() {
        // todo test reading invalid json files
    }

    #[test]
    fn json_component() {
        let cb: &dyn Component = &CircuitBreaker::new("test_cb");
        cb.to_json().unwrap();
        let cb = CircuitBreaker::from_json("test_cb").unwrap();
        assert_eq!(cb.name(), "test_cb");
        assert_eq!(cb.r#type(), ComponentType::CircuitBreaker);

        // todo test all components
    }
}
