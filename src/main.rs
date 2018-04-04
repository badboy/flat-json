extern crate serde;
extern crate serde_json;
extern crate atty;

use std::env;
use std::io::{self, Read};
use std::fs::File;
use std::fmt::Display;
use serde_json::Value;
use atty::Stream;

fn scalar<D: Display>(msg: D, prefix: Option<String>) {
    match prefix {
        Some(s) => println!("{}: {}", s, msg),
        None => println!("{}", msg)
    }
}

fn handle_value(value: &Value, prefix: Option<String>) {
    match value {
        &Value::Null => scalar("<null>", prefix),
        &Value::Bool(ref b) => scalar(b, prefix),
        &Value::Number(ref i) => scalar(i, prefix),
        &Value::String(ref s) => scalar(format!("{:?}", s), prefix),
        &Value::Array(ref v) => {
            let prefix = prefix.unwrap_or_else(|| "".to_string());
            for (idx, elem) in v.iter().enumerate() {
                let sub_prefix = format!("{}[{}]", prefix, idx);
                handle_value(elem, Some(sub_prefix));
            }
        }
        &Value::Object(ref map) => {
            let prefix = prefix.unwrap_or_else(|| "".to_string());
            for (key, value) in map {
                let sub_prefix = format!("{}.{}", prefix, key);
                handle_value(value, Some(sub_prefix));
            }
        }
    }
}

fn get_json() -> String {
    let args = env::args().skip(1).next();
    let arg = args.unwrap_or("".to_string());
    let use_stdin = arg == "-" || arg == "";

    let mut buffer = String::new();
    if !atty::is(Stream::Stdin) || use_stdin {
        io::stdin().read_to_string(&mut buffer).unwrap();
    } else {
        let mut f = File::open(arg).unwrap();
        f.read_to_string(&mut buffer).unwrap();
    }

    buffer
}

fn main() {
    let buffer = get_json();

    let data: Value = match serde_json::from_str(&buffer) {
        Ok(data) => data,
        Err(e) => {
            println!("input is not valid JSON: {}", e);
            std::process::exit(1);
        }
    };

    handle_value(&data, None);
}
