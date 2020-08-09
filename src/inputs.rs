use rand::{self, Rng};
use tinyset::Set64;

#[derive(Debug)]
pub struct Input {
    pub amount: usize,
    pub coins: Vec<usize>
}

pub fn generate(n: usize,
                amount_min: usize, amount_max: usize,
                coins_min: usize, coins_max: usize,
                nominal_max: usize) -> (String, Vec<Input>) {
    let label = format!(
        "minimal_number [amount from {} to {}, coins number from {} to {}, nominals up to {}, {} tests]",
        amount_min, amount_max, coins_min, coins_max, nominal_max, n);

    let mut inputs = vec![];

    let mut rng = rand::thread_rng();
    let amount = rng.gen_range(amount_min, amount_max + 1);
    let nominal_max = std::cmp::min(amount, nominal_max);

    for _ in 0..n {
        let mut coins = Set64::new();
        let coins_number = rng.gen_range(coins_min, coins_max + 1);
        while coins.len() < coins_number {
            coins.insert(rng.gen_range(1, nominal_max + 1));
        }

        inputs.push(Input {
            amount,
            coins: coins.drain().collect()
        });
    }

    (label, inputs)
}