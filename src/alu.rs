use super::gates::*;

#[derive(Debug)]
pub struct AdderResult {
    pub sum: bool,
    pub carry: bool
}

pub fn half_adder(x: bool, y: bool) -> AdderResult {
    let sum = xor(x, y);
    let carry = and(x, y);

    AdderResult { sum, carry }
}

pub fn full_adder(a: bool, b: bool, c: bool) -> AdderResult {
    let result = half_adder(a, b);
    let result1 = half_adder(result.sum, c);
    let result2 = half_adder(result.carry, result1.carry);

    AdderResult { sum: result1.sum, carry: result2.sum }
}

pub fn add_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
    let mut sum: [bool; 16] = [false; 16];
    let mut carries: [bool; 16] = [false; 16];
    let result = half_adder(x[0], y[0]);
    carries[0] = result.carry;
    sum[0] = result.sum;

    let mut counter = 0;

    for child in x {
        match counter {
            0 => counter += 1,
            _ => {
                let result = full_adder(child, y[counter], carries[counter - 1]);
                carries[counter] = result.carry;
                sum[counter] = result.sum;
                counter += 1;
            }
        }
    }
    sum
}
