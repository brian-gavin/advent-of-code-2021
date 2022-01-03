use std::{
    convert::TryFrom,
    io::{self, prelude::*},
};

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<(&str, &str)> for Point {
    type Error = std::num::ParseIntError;

    fn try_from(p: (&str, &str)) -> Result<Self, Self::Error> {
        let (x, y) = p;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug)]
struct Line(Point, Point);

fn max3(a: usize, b: usize, c: usize) -> usize {
    let mut m = a;
    if b > m {
        m = b;
    }
    if c > m {
        m = c;
    }
    m
}

struct Graph<'a>(&'a Vec<Vec<u32>>);

impl<'a> std::fmt::Display for Graph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in self.0.iter() {
            for v in row.iter() {
                if *v == 0 {
                    write!(f, "{}", '.')?;
                } else {
                    write!(f, "{}", *v)?;
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve() -> String {
    let stdin = io::stdin();
    let mut max_x = 0;
    let mut max_y = 0;
    let lines: Vec<Line> = stdin
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (p1, p2) = line
                .split_once(" -> ")
                .map(|(p1, p2)| (p1.split_once(',').unwrap(), p2.split_once(',').unwrap()))
                .unwrap();
            let (p1, p2): (Point, Point) = (p1.try_into().unwrap(), p2.try_into().unwrap());
            max_x = max3(max_x, p1.x, p2.x);
            max_y = max3(max_y, p1.y, p2.y);
            Line(p1, p2)
        })
        .collect();
    let g = plot(max_x, max_y, lines);
    println!("{}", Graph(&g));
    let mut points = Vec::new();
    for (y, row) in g.iter().enumerate() {
        for (x, cnt) in row.iter().enumerate() {
            if *cnt > 1 {
                points.push(Point { x, y })
            }
        }
    }
    format!("{}", points.len())
}

fn step(from: usize, to: usize) -> usize {
    use std::cmp::Ordering::*;
    match from.cmp(&to) {
        Equal => from,
        Greater => from - 1,
        Less => from + 1,
    }
}

fn step_point(from: &Point, to: &Point) -> Point {
    let x = step(from.x, to.x);
    let y = step(from.y, to.y);
    Point { x, y }
}

fn plot<L: IntoIterator<Item = Line>>(max_x: usize, max_y: usize, lines: L) -> Vec<Vec<u32>> {
    let mut g = vec![vec![0; max_x + 1]; max_y + 1];
    for line in lines.into_iter() {
        let Line(mut p1, p2) = line;
        g[p1.y][p1.x] += 1;
        while p1 != p2 {
            p1 = step_point(&p1, &p2);
            g[p1.y][p1.x] += 1;
        }
    }
    g
}
