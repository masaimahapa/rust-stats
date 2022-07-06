fn main() {
    let digits = vec![10.0, 2.0, 3.0, 4.0, 5.0, 8.0];
    println!("input : {:?}", digits);

    println!("count : {}", digits.len());
    let mean_number = stats::mean(&digits);
    println!("mean {}", mean_number);

    let v = stats::variance(&digits);
    println!("variance {}", v);

    let standard_deviation = stats::std(&digits);
    println!("standard deviation {}", standard_deviation);
}
