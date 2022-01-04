use std::{
    io::{self, BufRead},
    str::Chars,
};

pub fn solve() -> String {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|s| valid_chunks(&s))
        .filter_map(Result::err)
        .filter_map(|e| match e {
            ChunkError::Corrupt(c) => Some(c),
            ChunkError::Incomplete => None, // ignore for now
        })
        .map(points)
        .sum::<u32>()
        .to_string()
}

enum ChunkError {
    Incomplete,
    Corrupt(char),
}

#[must_use]
fn valid_chunks(s: &str) -> Result<(), ChunkError> {
    let mut it = s.chars();
    while let Some(c) = it.next() {
        chunk(&mut it, c)?;
    }
    Ok(())
}

fn must_next(it: &mut Chars) -> Result<char, ChunkError> {
    if let Some(n) = it.next() {
        Ok(n)
    } else {
        return Err(ChunkError::Incomplete);
    }
}

fn chunk(it: &mut Chars, start: char) -> Result<(), ChunkError> {
    if !is_opening(start) {
        return Err(ChunkError::Corrupt(start));
    }
    let mut n = must_next(it)?;
    while is_opening(n) {
        chunk(it, n)?;
        n = must_next(it)?;
    }
    if n != closing_of(start) {
        Err(ChunkError::Corrupt(n))
    } else {
        Ok(())
    }
}

const fn closing_of(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

const fn is_opening(c: char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

const fn points(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chunks() {
        assert!(matches!(
            valid_chunks("{([(<{}[<>[]}>{[]{[(<()>"),
            Err(ChunkError::Corrupt('}'))
        ));
        assert!(matches!(
            valid_chunks("[[<[([]))<([[{}[[()]]]"),
            Err(ChunkError::Corrupt(')')),
        ));
        assert!(matches!(
            valid_chunks("[{[{({}]{}}([{[{{{}}([]"),
            Err(ChunkError::Corrupt(']')),
        ));
        assert!(matches!(
            valid_chunks("[<(<(<(<{}))><([]([]()"),
            Err(ChunkError::Corrupt(')')),
        ));
        assert!(matches!(
            valid_chunks("<{([([[(<>()){}]>(<<{{"),
            Err(ChunkError::Corrupt('>'))
        ));
    }
}
