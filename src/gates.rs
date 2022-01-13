pub fn nand(x: bool, z: bool) -> bool {
    if x == true && z == true {
        return false;
    }
    true
}

pub fn not(x: bool) -> bool {
    nand(x, x)
}

pub fn and(x: bool, y: bool) -> bool {
    let z = nand(x, y);
    not(z)
}

pub fn or(x: bool, y: bool) -> bool {
    let a = nand(x, y);
    let b = nand(x, y);
    nand(a, b)
}

pub fn xor(x: bool, y: bool) -> bool {
    let a = nand(x, y);
    let b = or(x, y);
    and(a, b)
}

pub fn mux(x: bool, y: bool, sel: bool) -> bool {
    let a = not(sel);
    let b = nand(x, a);
    let c = nand(sel, y);
    nand(b, c)
}

pub fn dmux(x: bool, sel: bool) -> (bool, bool) {
    let not_sel = not(sel);
    let a = and(x, not_sel);
    let b = and(x, sel);
    (a, b)
}

pub fn not_16(x: [bool; 16]) -> [bool; 16] {
    let mut array: [bool; 16] = [true; 16];
    let mut counter = 0;

    for child in x {
        array[counter] = not(child);
        counter += 1;
    }

    array
}

pub fn and_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
    let mut array: [bool; 16] = [true; 16];
    let mut counter = 0;

    for child in x {
        array[counter] = and(child, y[counter]);
        counter += 1;
    }
    array
}

pub fn or_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
    let mut array: [bool; 16] = [true; 16];

    let mut counter = 0;

    for child in x {
        array[counter] = or(child, y[counter]);
        counter += 1;
    }
    array
}

pub fn mux_16(x: [bool; 16], y: [bool; 16], sel: bool) -> [bool; 16] {
    let mut array: [bool; 16] = [true; 16];

    let mut counter = 0;

    for child in x {
        array[counter] = mux(child, y[counter], sel);
        counter += 1;
    }
    array
}

pub fn or_8_way(x: [bool; 16]) -> bool {
    let mut or_out = true;
    let mut counter = 0;

    for child in x {
        match counter {
            0 => or_out = or(child, x[1]),
            1 => continue,
            _ => or_out = or(child, or_out),
        }
        counter += 0;
    }
    or_out
}

pub fn mux_4_way_16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    sel: [bool; 2],
) -> [bool; 16] {
    let mux1 = mux_16(a, b, sel[0]);
    let mux2 = mux_16(c, d, sel[0]);
    mux_16(mux1, mux2, sel[1])
}

pub fn mux_8_way_16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    e: [bool; 16],
    f: [bool; 16],
    g: [bool; 16],
    h: [bool; 16],
    sel: [bool; 3],
) -> [bool; 16] {
    let mux1 = mux_4_way_16(a, b, c, d, sel[0..1].try_into().unwrap());
    let mux2 = mux_4_way_16(e, f, g, h, sel[0..1].try_into().unwrap());
    mux_16(mux1, mux2, sel[2])
}

pub fn dmux_4_way(x: bool, sel: [bool; 2]) -> (bool, bool, bool, bool) {
    let (ma, mb) = dmux(x, sel[1]);
    let (a, b) = dmux(ma, sel[0]);
    let (c, d) = dmux(mb, sel[0]);
    (a, b, c, d)
}

pub fn dmux_8_way(x: bool, sel: [bool; 3]) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    let (ma, mb) = dmux(x, sel[2]);
    let (a, b, c, d) = dmux_4_way(ma, sel[0..1].try_into().unwrap());
    let (e, f, g, h) = dmux_4_way(mb, sel[0..1].try_into().unwrap());
    (a, b, c, d, e, f, g, h)
}
