use std::collections::HashSet;

pub fn main() {
    println!("Coin Change");
    let aaa = coin_change_dp(vec![1, 2, 5], 11);
    println!("{aaa}");
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct State {
    amount: i32,
    coins: i32,
}

// O(n) Memoizing
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    use std::collections::VecDeque;

    let mut answer = i32::MAX;

    let mut found = false;

    let mut deque = VecDeque::new();

    let mut visited = HashSet::new();

    let start = State::default();

    deque.push_front(start);

    while let Some(state) = deque.pop_front() {
        if state.amount == amount {
            found = true;
            answer = answer.min(state.coins);
            continue;
        }

        if state.amount > amount {
            continue;
        }

        for idx in 0..coins.len() {
            let mut clone = state;
            clone.coins += 1;
            clone.amount += coins[idx];

            if !visited.contains(&clone) {
                visited.insert(clone);
                deque.push_front(clone);
            }
        }
    }

    if found {
        answer
    } else {
        -1
    }
}

// DP O(n2) fill all values ab to the wanted amount
pub fn coin_change_dp(coins: Vec<i32>, amount: i32) -> i32 {
    const BIG_NUM: i32 = 10000;
    let amount = amount as usize;

    let mut dp = vec![BIG_NUM; amount + 1];

    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins.iter() {
            let coin = coin as usize;
            if coin <= i {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }

    dbg!(&dp);

    if dp[amount] < BIG_NUM {
        dp[amount]
    } else {
        -1
    }
}
