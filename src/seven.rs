use std::io;

fn cost(from: u64, to: u64) -> u64 {
    let (lo, hi) = if from < to { (from, to) } else { (to, from) };
    let distance = hi - lo;
    (1..=distance).sum::<u64>()
}

pub fn solve() -> String {
    let mut crabs = crate::parse_csv::<u64>(io::stdin()).unwrap();
    crabs.sort();

    let max_pos = crabs.last().unwrap().clone();
    let min_alignment = (0..=max_pos).fold(u64::MAX, |min, align_pos| {
        u64::min(
            crabs
                .iter()
                .map(|crab_pos| cost(*crab_pos, align_pos))
                .sum(),
            min,
        )
    });
    format!("{}", min_alignment)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cost() {
        let td = [
            (1, 5, 10),
            (16, 5, 66),
            (2, 5, 6),
            (0, 5, 15),
            (4, 5, 1),
            (2, 5, 6),
            (7, 5, 3),
            (1, 5, 10),
            (2, 5, 6),
            (14, 5, 45),
        ];
        for (from, to, fuel) in td {
            assert_eq!(
                cost(from, to),
                fuel,
                "cost(from: {}, to: {}) != {}",
                from,
                to,
                fuel
            );
        }
    }
}
