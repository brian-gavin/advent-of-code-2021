use std::{
    collections::HashSet,
    io::{prelude::*, stdin},
};

pub fn solve() -> String {
    let input = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().filter_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<Vec<_>>>();
    let low_points = find_low_points(&input);
    let basins = find_largest_basins(&input, low_points);
    dbg!(&basins);
    format!("{}", basins.into_iter().product::<usize>())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    i: usize,
    j: usize,
    v: u32,
}

fn find_low_points(input: &Vec<Vec<u32>>) -> Vec<Point> {
    let mut low_points = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            let p = Point { i, j, v: *v };
            if neighbors(&p, input).into_iter().all(|n| n.v > p.v) {
                low_points.push(p);
            }
        }
    }
    low_points
}

fn find_largest_basins(input: &Vec<Vec<u32>>, low_points: Vec<Point>) -> Vec<usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut basins = BinaryHeap::with_capacity(3);
    for coords in low_points.into_iter() {
        let basin = build_basin(coords, input);
        if basins.len() == 3 {
            basins.push(Reverse(basin));
            let _ = basins.pop();
        } else {
            basins.push(Reverse(basin));
        }
    }
    basins.into_iter().map(|Reverse(n)| n).collect()
}

// neighbors: above, left, right, below
fn neighbors(p: &Point, input: &Vec<Vec<u32>>) -> Vec<Point> {
    let get = |i: usize, j: usize| {
        let v = input
            .get(i)
            .and_then(|row| row.get(j).cloned())
            .unwrap_or(9);
        Point { i, j, v }
    };
    let Point { i, j, .. } = p;
    let above = get(i.wrapping_sub(1), *j);
    let left = get(*i, j.wrapping_sub(1));
    let right = get(*i, j + 1);
    let below = get(i + 1, *j);
    vec![above, left, right, below]
}

fn build_basin(start: Point, input: &Vec<Vec<u32>>) -> usize {
    let mut basin = HashSet::new();
    basin.insert(start.clone());
    neighbors(&start, input)
        .into_iter()
        .filter(|n| n.v != 9) // "out of bounds" have the value 9
        .for_each(|n| grow_basin(input, &mut basin, n));
    basin.len()
}

fn grow_basin(input: &Vec<Vec<u32>>, basin: &mut HashSet<Point>, p: Point) {
    if basin.get(&p).is_some() {
        return;
    }
    basin.insert(p.clone());
    neighbors(&p, input)
        .into_iter()
        .filter(|n| n.v != 9 && n.v > p.v)
        .for_each(|n| grow_basin(input, basin, n));
}
