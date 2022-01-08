use std::{
    io::{self, BufRead},
    str::Chars,
};

pub fn solve() -> String {
    let mut scores: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .filter_map(|s| valid_chunks(&s).err())
        .filter_map(|e| match e {
            ChunkError::Incomplete(fixes) => Some(fixes),
            _ => None,
        })
        .map(|fixes| {
            fixes
                .into_iter()
                .fold(0, |score, fix| (score * 5) + points(fix))
        })
        .collect();
    scores.sort();
    let mid = scores[scores.len() / 2];
    mid.to_string()
}

#[derive(Debug)]
enum ChunkError {
    Incomplete(Vec<char>),
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

fn must_next(it: &mut Chars, expect: char) -> Result<char, ChunkError> {
    if let Some(n) = it.next() {
        Ok(n)
    } else {
        return Err(ChunkError::Incomplete(vec![expect]));
    }
}

fn chunk(it: &mut Chars, start: char) -> Result<(), ChunkError> {
    use ChunkError::*;
    if !is_opening(start) {
        return Err(ChunkError::Corrupt(start));
    }
    let closing = closing_of(start);
    let mut n = must_next(it, closing)?;
    while is_opening(n) {
        chunk(it, n).map_err(|e| match e {
            Incomplete(mut v) => {
                v.push(closing);
                Incomplete(v)
            }
            e => e,
        })?;
        n = must_next(it, closing)?;
    }
    if n != closing {
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

#[allow(dead_code)]
const fn points1(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

const fn points(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
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
