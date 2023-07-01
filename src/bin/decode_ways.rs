pub fn main() {
    println!("Decode Ways");
}

// O(n) space O(1)
pub fn num_decodings(s: String) -> i32 {
    // special cases
    if s.as_bytes()[0] == b'0' {
        return 0;
    }
    if s.len() == 1 {
        return 1;
    }

    //num of decoding in case adding one or two letters
    let (mut decode_1, mut decode_2) = (1, 1);

    for pair in s.as_bytes().windows(2) {
        let mut n = 0;

        if pair[1] != b'0' {
            n += decode_1;
        }

        if (pair[0] == b'1') || (pair[0] == b'2' && pair[1] - b'0' <= 6) {
            n += decode_2;
        }

        decode_2 = decode_1;
        decode_1 = n;
    }

    decode_1
}

// O(n) space O(1)
pub fn num_decodings_chars(s: String) -> i32 {
    // special cases
    if s.chars().nth(0).unwrap() == '0' {
        return 0;
    }

    let (mut prev, mut curr) = (1, 1);

    for i in 2..s.len() + 1 {
        let curr_char = s.chars().nth(i - 1).unwrap();
        let prev_char = s.chars().nth(i - 2).unwrap();

        let p_one = if curr_char != '0' { curr } else { 0 };
        let p_two = if prev_char == '1' || (prev_char == '2' && curr_char <= '6') {
            prev
        } else {
            0
        };

        prev = curr;
        curr = p_one + p_two;
    }

    curr
}
