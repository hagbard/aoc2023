use std::ops::Mul;

use ndarray::{Array, Ix1, s};
use num_integer::Integer;

// 0: a0        a1          a2          a3          a4         a5
// 1:   a1-a0       a2-a1       a3-a2       a4-a3       a5-a4
// 2:       a2-2a1+a0   a3-2a2+a1   a4-2a3+a2   a5-2a4+a3
// 3:         a3-3a2+3a1-a0 a4-3a3+3a2-a1 a5-3a4+3a3-a2
// 4:          a4-4a3+6a2-4a1+a0     a5-4a4+6a3-4a2+a1
// 5:                  a5-5a4+10a3-10a2+5a1-a0
//
// R0: (0C0)
// R1: (1C0, -1C1)
// R2: (2C0, -2C1, +2C2)
//
// Next = SUM(Last value in each row)
//
// (1+1+1+1+1+1).a5 - (1+2+3+4+5).a4 + (1+3+6+10).a3 - (1+4+10).a2 + (1+5).a1 - (1).a0
// 6th 1-simplex      5th 2-simplex    4th 3-simplex   3rd 4-simplex 2nd 5-simplex
//
// For N inputs a0...aN-1
// aN == S(1, N).aN-1 - S(2, N-1).aN-2 + ... +/- S(N, 1).a0
//
// Generate sequence of coefficients from progressively shorter simplex sequences.
// Sum adjacent results up to one before the final value, alternate +/- for last result.
//
// 0   1   2   3   4   5
// ------------------------vvv
// 1,  2,  3,  4,  5,  6    6
// 1,  3,  6, 10, 15      -15
// 1,  4, 10, 20           20
// 1,  5, 15              -15
// 1,  6                    6
// 1                       -1
//
// a6 = -1.a0 + 6.a1 - 15.a2 + 20.a3 - 15.a4 + 6.a5

pub fn run(input: &str) -> (i64, i64) {
    let lines: Vec<Array<i64, Ix1>> = input.lines()
        .map(|line| line.split_whitespace().map(atoi64).collect::<Array<i64, Ix1>>())
        .collect();

    let size = lines[0].len();
    let mut coeff: Array<i64, Ix1> = (1..=size as i64).collect();
    let mut end = size;
    while end > 1 {
        end -= 1;
        for i in 1..end {
            coeff[i] = coeff[i - 1] + coeff[i];
        }
        // Determine +/- factor from END of coefficients (final coefficient is always +)
        if (size - end).is_odd() {
            coeff[end - 1] = -coeff[end - 1]
        }
    }

    (lines.iter().map(|line| line.mul(&coeff).sum()).sum::<i64>(),
     lines.iter().map(|line| line.slice(s![..;-1]).mul(&coeff).sum()).sum::<i64>())
}

fn atoi64(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}
