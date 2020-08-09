pub mod minimal_number;
pub mod optimal_change;
pub mod inputs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let (_, inputs) = inputs::generate(1000, 50, 100, 2, 7, 100);

        inputs.into_iter().for_each(|input| {
            println!("Input: {:?}", input);
            let number = minimal_number::calculate(input.amount, &input.coins);
            let solution = optimal_change::calculate(input.amount, &input.coins);
            println!("Solution: {:?}", solution);
            println!("Number: {:?}", number);
            println!();

            assert_eq!(solution.map(|v| v.len()), number);
        })
    }
}