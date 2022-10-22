/// 偏移量
const BIAS: i64 = 1023;
/// 底数
const RADIX: f64 = 2.0;

fn main() {
    println!("\n<--------------->");
    let x: f64 = 0.0006;
    pretty_print(x);

    println!("\n<--------------->");
    let y: f64 = 0.0475;
    pretty_print(y);

    println!("\n<--------------->");
    pretty_print(x + y);
}

fn pretty_print(n: f64) {
    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, RADIX.powf(exp_ as f64), mant);

    println!("{} -> {}", n, n_);
    println!("field | as bits | as real number");
    println!("sign | {:01b} | {}", sign, sign_);
    println!("exp | {:011b} | {}", exp, exp_);
    println!("mantissa | {:052b} | {}", frac, mant);
}

fn to_parts(n: f64) -> (u64, u64, u64) {
    let bits = n.to_bits();

    let sign = (bits >> 63) & 1;
    let exponent = (bits >> 52) & 0b11111111111;
    let fraction = bits & 0b1111111111111111111111111111111111111111111111111111;

    (sign, exponent, fraction)
}

fn decode(sign: u64, exponent: u64, fraction: u64) -> (f64, i64, f64) {
    let signed_1 = (-1.0_f64).powf(sign as f64);

    let exponent = (exponent as i64) - BIAS;
    // let exponent = RADIX.powf(exponent as f64);

    let mut mantissa: f64 = 1.0;
    for i in 0..52 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f64;
            let weight: f64 = 2_f64.powf(i_ - 52.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

fn from_parts(sign: f64, exponent: f64, mantissa: f64) -> f64 {
    sign * exponent * mantissa
}
