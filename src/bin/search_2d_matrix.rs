pub fn main() {
    println!("Search a 2D Matrix");
}

// Log(n*m) best memory consumption
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    use std::cmp::Ordering::*;

    let mut row = None;

    let (mut top, mut bottom) = (0, matrix.len());

    while top < bottom {
        let mid = (top + bottom) / 2;

        match (
            matrix[mid].first().unwrap().cmp(&target),
            matrix[mid].last().unwrap().cmp(&target),
        ) {
            (Equal, _) | (_, Equal) => return true,
            (Less, Greater) => {
                row = Some(mid);
                break;
            }
            (Less, Less) => top = mid + 1,
            (Greater, Greater) => bottom = mid,
            (Greater, Less) => unreachable!(),
        }
    }

    if row.is_none() {
        return false;
    }

    let row = &matrix[row.unwrap()];

    let mut left = 0;
    let mut right = row.len();

    while left < right {
        let mid = (left + right) / 2;

        match row[mid].cmp(&target) {
            Equal => return true,
            Less => left = mid + 1,
            Greater => right = mid,
        }
    }

    false
}

// Log(n*m)
pub fn search_matrix_built_in_concat(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    matrix.concat().binary_search(&target).is_ok()
}

// Log(n*m)
pub fn search_matrix_built_in(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let cmp = matrix.binary_search_by(|slice| slice[0].cmp(&target));

    match cmp {
        Ok(_) => true,
        Err(0) => false,
        Err(index) => matrix[index - 1].binary_search(&target).is_ok(),
    }
}
