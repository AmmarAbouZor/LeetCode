pub fn main() {
    println!("Roman to Int");
}

// The solution is to read the values backwards and compare the accumulated value
// to determine if the value is negative of positive
// Time: O(n) Space O(1)
pub fn roman_to_int(s: String) -> i32 {
    s.as_bytes().iter().rfold(0, |acc, ch| {
        acc + match ch {
            b'I' => {
                if acc >= 5 {
                    -1
                } else {
                    1
                }
            }
            b'V' => 5,
            b'X' => {
                if acc >= 50 {
                    -10
                } else {
                    10
                }
            }
            b'L' => 50,
            b'C' => {
                if acc >= 500 {
                    -100
                } else {
                    100
                }
            }
            b'D' => 500,
            b'M' => 1000,
            _ => unreachable!(),
        }
    })
}

// This solution is simple and direct. It has modelling a lot and doesn't have tweaks
// Time: O(n)
mod final_soltion {
    use std::char;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Roman {
        I,
        V,
        X,
        L,
        C,
        D,
        M,
    }

    impl Roman {
        #[inline]
        fn num(self) -> i32 {
            match self {
                Roman::I => 1,
                Roman::V => 5,
                Roman::X => 10,
                Roman::L => 50,
                Roman::C => 100,
                Roman::D => 500,
                Roman::M => 1000,
            }
        }

        fn next_reduce(self) -> &'static [Self] {
            match self {
                Roman::I => &[Roman::V, Self::X],
                Roman::X => &[Roman::L, Roman::C],
                Roman::C => &[Roman::D, Roman::M],
                Roman::V | Roman::L | Roman::D | Roman::M => &[],
            }
        }
    }

    impl From<char> for Roman {
        fn from(value: char) -> Self {
            match value {
                'I' => Roman::I,
                'V' => Roman::V,
                'X' => Roman::X,
                'L' => Roman::L,
                'C' => Roman::C,
                'D' => Roman::D,
                'M' => Roman::M,
                invalid => panic!("Invalid char: {invalid}"),
            }
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let romans: Vec<Roman> = s.chars().map(|ch| ch.into()).collect();
        let mut sum = 0;

        for i in 0..romans.len() {
            let current = romans[i];
            let next = romans.get(i + 1);
            if next.is_some_and(|n| current.next_reduce().contains(n)) {
                sum -= current.num();
            } else {
                sum += current.num();
            }
        }

        sum
    }
}
