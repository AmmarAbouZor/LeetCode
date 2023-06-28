pub fn main() {
    println!("Best time to buy and sell stocks");
}

// O(n^2)
pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .take(prices.len() - 1)
        .enumerate()
        .map(|(i, &num)| prices[i..].iter().max().unwrap() - num)
        .max()
        .unwrap()
}

// time O(n) space: O(1)
pub fn max_profit_eff(prices: Vec<i32>) -> i32 {
    let (mut profit, mut buy) = (0, prices[0]);

    for i in 1..prices.len() {
        profit = profit.max(prices[i] - buy);
        buy = buy.min(prices[i]);
    }

    profit
}

pub fn max_profit_eff_one_line(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((i32::MAX, 0), |(min, max_profit), &el| {
            (el.min(min), max_profit.max(el - min))
        })
        .1
}
