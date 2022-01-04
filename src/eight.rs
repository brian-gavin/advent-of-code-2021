use std::{io::{self, prelude::*}, collections::HashMap, str::Chars};

pub fn solve() -> String {
    let input = {
        let mut s = String::new();
        io::stdin().lock().read_to_string(&mut s).unwrap();
        s
    };
    format!("{}", input.lines().map(solve_entry).map(|s| s.parse::<usize>().unwrap()).sum::<usize>())
}

// associate array of a number to its used segments
const SEGMENTS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn parse_line(s: &str) -> (Vec<&str>, Vec<&str>) {
    fn space_sep(s: &str) -> Vec<&str> {
        s.trim().split(' ').collect()
    }
    let (patterns, digits) = s.trim().split_once(" | ").unwrap();
    (space_sep(patterns), space_sep(digits))
}

fn determine_bef(patterns: &[&str]) -> (char,char,char) {
    patterns.iter()
    .flat_map(|s| s.chars())
    .fold(HashMap::new(), |mut m, c| {
        m.insert(c, m.get(&c).map_or(1, |n| n + 1));
        m
    })
    .into_iter()
    .fold((' ',' ',' '), |(b,e,f),(k,v)| match v {
        6 => (k,e,f),
        4 => (b,k,f),
        9 => (b,e,k),
        _ => (b,e,f),
    })
}

fn not_contains(mut p: Chars, chars: &[char]) -> char {
    p.find(|c| !chars.contains(c)).unwrap()
}

fn patterns_one_four_seven_eight<'a>(p: &'a[&str]) -> (Chars<'a>,Chars<'a>,Chars<'a>,Chars<'a>) {
    let it = p.iter();
    let of_length = |l: usize| it.clone().filter(|p|p.len() == l).map(|s|s.chars()).nth(0).unwrap();
    (of_length(2),of_length(4),of_length(3),of_length(7))
}

fn derandomize<'a>(a:char,b:char,c:char,d:char,e:char,f:char,g:char) -> impl FnMut(&'a str)->String {
    move |s:&str| {
        let mut v: Vec<char> = s.chars().map(|cc| if cc == a {'a'} else if cc == b { 'b' } else if cc == c {'c'} else if cc == d {'d'} else if cc == e {'e' } else if cc == f {'f'} else if cc == g {'g'} else {unreachable!()}).collect();
        v.sort();
        String::from_iter(v.into_iter())
    }
}

fn to_segment(s: &str) -> String {
    SEGMENTS.iter().enumerate().find_map(|(i, segment)| if s == *segment {Some(i.to_string())} else {None}).unwrap()
}

// returns how many times 1,4,7,8 appear in s
fn solve_entry(s: &str) -> String {
    let (patterns, digits) = parse_line(s);
    let (one,four,seven,eight) = patterns_one_four_seven_eight(&patterns);
    let (b,e,f) = determine_bef(&patterns);
    let c = not_contains(one, &[f]);
    let a = not_contains(seven, &[c, f]);
    let d = not_contains(four, &[b,c,f]);
    let g = not_contains(eight, &[a,b,c,d,e,f]);

    let digits = digits.into_iter()
    .map(derandomize(a,b,c,d,e,f,g))
    .map(|s| to_segment(&s))
    .collect::<Vec<_>>();
    digits.as_slice().join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_entry() {}

    #[test]
    fn test_parse_line() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let (patterns, digits) = parse_line(input);
        assert_eq!(
            patterns,
            vec![
                "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd",
                "edb"
            ]
        );
        assert_eq!(digits, vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]);
    }
}
