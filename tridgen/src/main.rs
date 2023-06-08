use rand::Rng;
use trid::TurkishId;

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let seq: u32 = rng.gen_range(100_000_000..1_000_000_000);
        println!("{}", TurkishId::from_seq(seq).unwrap());
    }
}
