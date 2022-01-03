fn main() {
    use aoc2021::six::fishes_slow;
    println!("     0 1 2 3 4 5 6 7 8");
    println!(" ---------------------");
    for day in 0..=20 {
        print!("{}  | ", day);
        for i in 0..=8 {
            print!("{} ", fishes_slow(vec![i], day).len() - 1);
        }
        println!();
    }
}
