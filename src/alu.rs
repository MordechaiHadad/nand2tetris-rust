use super::gates::*;

#[derive(Debug)]
pub struct AdderResult {
    pub sum: bool,
    pub carry: bool,
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

    AdderResult {
        sum: result1.sum,
        carry: result2.sum,
    }
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

pub fn increment_16(x: [bool; 16]) -> [bool; 16] {
    let y = [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    add_16(x, y)
}

#[derive(Debug)]
pub struct AluResult {
    out: [bool; 16],
    zr: bool,
    ng: bool,
}
pub fn alu(
    x: [bool; 16],
    y: [bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> AluResult {
    let mx = mux_16(x, [false; 16], zx);
    let notmx = not_16(mx);
    let x1 = mux_16(mx, notmx, nx);

    let my = mux_16(y, [false; 16], zy);
    let notmy = not_16(my);
    let y1 = mux_16(my, notmy, ny);

    let xplusy = add_16(x1, y1);
    let xandy = and_16(x1, y1);
    let fout = mux_16(xandy, xplusy, f);

    let notfout = not_16(fout);
    let out = mux_16(fout, notfout, no);

    let not15 = out[15];
    let firstor: [bool; 8] = out[0..8].try_into().unwrap();
    let firstor = [firstor, [false; 8]].concat().try_into().unwrap();
    let secondor: [bool; 8] = out[8..16].try_into().unwrap();
    let secondor = [secondor, [false; 8]].concat().try_into().unwrap();

    let zr1 = or_8_way(firstor);
    let hi = [false; 8];
    let zr2 = or_8_way(secondor);
    let nzr = or(zr1, zr2);
    let zr = not(nzr);

    let ng = and(not15, true);

    AluResult { out, zr, ng }
}
