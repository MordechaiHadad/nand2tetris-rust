mod gates;
mod alu;

fn main() {

    let x = [false; 16];
    let y = [true; 16];

    let result = alu::alu(x, y, false, true, false, true, false, true);

    println!("{:?}", result);
}
