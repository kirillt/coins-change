//todo: tinyset

pub fn calculate(amount: usize, coins: &Vec<usize>) -> Option<usize> {
    let mut table: Vec<Option<usize>> = vec![None; amount + 1];
    table[0] = Some(0);

    fn recursive(amount: usize, coins: &Vec<usize>, table: &mut Vec<Option<usize>>) -> Option<usize> {
        match table[amount] {
            Some(min) => Some(min),
            None => {
                let mut min = None;
                for coin in coins.iter() {
                    if amount >= *coin {
                        let result = recursive(amount - *coin, coins, table);
                        if let Some(result) = result {
                            min = Some(if let Some(suboptimal) = min {
                                std::cmp::min(suboptimal, result)
                            } else {
                                result
                            });
                        }
                    }
                }

                min = min.map(|min| min + 1);
                table[amount] = min;
                min
            }
        }
    };

    recursive(amount, &coins, &mut table)
}