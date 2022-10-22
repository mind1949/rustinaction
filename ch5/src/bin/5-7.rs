fn main() {
    let n: f32 = 42.42;
    let n_bits = n.to_bits();
    println!("{:032b} :bit pattern", n_bits);

    println!("\n分离符号位");
    let sign_bit = n_bits >> 31;
    println!("{:01b} :sign", sign_bit);

    println!("\n分离指数位置");
    let exponent = n_bits >> 23;
    println!("{:b} :exponent", exponent);
    let mask = 0xff;
    println!("{:b} :mask", mask);
    let exponent = exponent & mask;
    println!("{:b} :exponent & mask", exponent);
    let exponent = (exponent as i32) - 127;
    println!("{:08b} :(exponent & mask) - 127", exponent);

    println!("\n分离尾数");
    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    println!("{} :mantissa", mantissa);
}
