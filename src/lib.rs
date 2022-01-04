use std::{
    io::{prelude::*, Stdin},
    str::FromStr,
};

pub mod eight;
pub mod five;
pub mod nine;
pub mod seven;
pub mod six;

pub fn parse_csv<T>(stdin: Stdin) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
{
    let mut s = String::new();
    stdin.lock().read_to_string(&mut s).unwrap();
    s.trim().split(',').map(|n| n.parse()).collect()
}
