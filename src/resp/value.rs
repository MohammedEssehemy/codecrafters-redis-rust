use anyhow::{Error, Result};
use bytes::BytesMut;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Value {
    /// encoded  "+OK\r\n"
    SimpleString(String),
    /// encoded "$5\r\nhello\r\n"
    BulkString(String),
    /// Null bulk reply, `$-1\r\n`
    Null,
    /// encoded "-Error message\r\n"
    Error(String),
    /// encoded ":0\r\n"
    Integer(i64),
    /// encoded "*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
    Array(Vec<Value>),
}

impl Value {
    pub fn to_command(&self) -> Result<(String, Vec<Value>)> {
        match self {
            Value::Array(items) => {
                return Ok((
                    items.first().unwrap().unwrap_bulk(),
                    items.clone().into_iter().skip(1).collect(),
                ));
            }
            _ => Err(Error::msg("not an array")),
        }
    }

    fn unwrap_bulk(&self) -> String {
        match self {
            Value::BulkString(str) => str.clone(),
            _ => panic!("not a bulk string"),
        }
    }

    pub fn encode(&self) -> String {
        match &self {
            Value::Null => "$-1\r\n".to_string(),
            Value::SimpleString(s) => format!("+{}\r\n", s),
            Value::Error(msg) => format!("-{}\r\n", msg),
            Value::Integer(i) => format!(":{}\r\n", i),
            Value::BulkString(s) => format!("${}\r\n{}\r\n", s.chars().count(), s),
            Value::Array(s) => format!(
                "*{}\r\n{}",
                s.len(),
                s.iter().map(|v| v.encode()).collect::<String>()
            ),
        }
    }
}

const CARRIAGE_RETURN: u8 = b'\r';
const LINE_FEED: u8 = b'\n';

pub fn parse_message(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
    if buffer.is_empty() {
        return Ok(None);
    }
    match buffer[0] as char {
        '+' => decode_simple_string(buffer),
        ':' => decode_integer(buffer),
        '$' => decode_bulk_string(buffer),
        '*' => decode_array(buffer),
        _ => Err(Error::msg("unrecognised message type")),
    }
}

fn decode_integer(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
    if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
        let integer = parse_integer(line)?;

        Ok(Some((Value::Integer(integer), len + 1)))
    } else {
        Ok(None)
    }
}

fn decode_simple_string(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
    if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
        let str = parse_string(line)?;

        Ok(Some((Value::SimpleString(str), len + 1)))
    } else {
        Ok(None)
    }
}

fn decode_bulk_string(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
    let (bulk_length, mut bytes_consumed) = if let Some((line, len)) = read_until_crlf(&buffer[1..])
    {
        let bulk_length = parse_integer(line)? as usize;

        (bulk_length, len + 1)
    } else {
        return Ok(None);
    };

    let bulk_string = read_until_crlf(&buffer[bytes_consumed..]);
    if bulk_string.is_none() {
        return Err(Error::msg("missing bulk string"));
    }
    let (bulk_string, bulk_string_len) = bulk_string.unwrap();
    bytes_consumed += bulk_string_len;
    if bulk_string.len() != bulk_length {
        return Err(Error::msg("bulk string data not mactching length"));
    }

    Ok(Some((
        Value::BulkString(parse_string(bulk_string)?),
        bytes_consumed,
    )))
}

fn decode_array(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
    let (array_length, mut bytes_consumed) =
        if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
            let array_length = parse_integer(line)? as usize;

            (array_length, len + 1)
        } else {
            return Ok(None);
        };

    let mut items: Vec<Value> = Vec::new();
    for _ in 0..array_length {
        if let Some((v, len)) = parse_message(BytesMut::from(&buffer[bytes_consumed..]))? {
            items.push(v);
            bytes_consumed += len
        } else {
            return Err(Error::msg("array elements not mactching array length"));
        }
    }

    return Ok(Some((Value::Array(items), bytes_consumed)));
}

fn read_until_crlf(buffer: &[u8]) -> Option<(&[u8], usize)> {
    for i in 0..buffer.len() - 1 {
        if buffer[i] == CARRIAGE_RETURN && buffer[i + 1] == LINE_FEED {
            let buf_till_crlf = &buffer[0..i];
            return Some((buf_till_crlf, buf_till_crlf.len() + 2)); // CR + LF
        }
    }

    return None;
}

fn parse_string(bytes: &[u8]) -> Result<String> {
    String::from_utf8(bytes.to_vec()).map_err(|_| Error::msg("Could not parse string"))
}

fn parse_integer(bytes: &[u8]) -> Result<i64> {
    let str_integer = parse_string(bytes)?;
    (str_integer.parse::<i64>()).map_err(|_| Error::msg("Could not parse integer"))
}
