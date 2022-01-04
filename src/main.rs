use aoc2021::*;

fn main() {
    let problem = std::env::args().nth(1).unwrap_or_default();
    let answer = match problem.as_str() {
        "5" => five::solve(),
        "6" => six::solve(),
        "7" => seven::solve(),
        "8" => eight::solve(),
        p => {
            eprintln!("invalid problem: {}", p);
            std::process::exit(1);
        }
    };
    println!("problem {}: {}", problem, answer)
}
