use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]

struct Args {
    /// Probability of each event occurring on its own
    #[clap(short, long)]
    probability: f32,

    /// Number of times each event will occur
    #[clap(short, long)]
    count: u32,
}

fn calculate_probability(p: f32, c: u32) -> f32 {
    match c {
        0 => 0.0,
        1 => p,
        2 => p + p - (p * p),
        _ => {
            let prev = calculate_probability(p, c - 1);
            prev + p - (prev * p)
        },
    }
}

fn round(n: f32) -> f32 {
    (n * 100.0).round() / 100.0
}

fn main() {
    let args = Args::parse();
    let probability = calculate_probability(args.probability, args.count);
    println!("\nThe probability of any of {} events occuring where each event has probability {} is: \n\t{}", args.count, round(args.probability), round(probability))
}
