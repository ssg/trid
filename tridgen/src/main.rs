use clap::Parser;
use rand::Rng;
use trid::TurkishId;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(default_value_t = 1)]
    /// Number of ID numbers to be generated.
    count: u32,
}

fn main() {
    let args = Args::parse();
    let count = args.count;
    let mut rng = rand::rng();
    for _ in 0..count {
        let seq: u32 = rng.random_range(TurkishId::SEQ_RANGE);
        println!("{}", TurkishId::from_seq(seq).unwrap());
    }
}
