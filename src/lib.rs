pub fn mean(numbers: & Vec<f32>) -> f32{
    let numbers_iter = numbers.iter();
    let sum : f32= numbers_iter.sum();
    let length = numbers.len();
    // sum/numbers_iter.len()
    sum/ length as f32
}

pub fn std(numbers : &Vec<f32>) -> f32{
    let v = variance(&numbers);
    f32::sqrt(v)
}

pub fn variance(numbers : &Vec<f32>)-> f32{
    let length = numbers.len();
    let mut v: f32 = 0.0;
    let average = mean(&numbers);

    for num in numbers{
        v=v+ (num - average) * (num - average);
    }
    v / (length-1) as f32
}