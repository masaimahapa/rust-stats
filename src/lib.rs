//! #  Easy Stats
//!
//! `easy_stats` is a collection of utility functions which make 
//! performing descriptive statistics easy


/// Calculate the mean from a list of numbers
///
/// # Examples
///
/// ```
/// let arg = vec![1.0,2.0,3.0,4.0, 5.0];
/// let average = easy_stats::mean(&arg);
///
/// assert_eq!(3.0, average);
/// ```
pub fn mean(numbers: & Vec<f32>) -> f32{
    let numbers_iter = numbers.iter();
    let sum : f32= numbers_iter.sum();
    let length = numbers.len();
    // sum/numbers_iter.len()
    sum/ length as f32
}

/// Calculate the standard deviation of a list of numbers
///
/// # Examples
///
/// ```
/// let arg = vec![1.0,2.0,3.0,4.0, 5.0];
/// let standard_dev = easy_stats::std(&arg);
///
/// assert_eq!(1.5811388, standard_dev);
/// ```
pub fn std(numbers : &Vec<f32>) -> f32{
    let v = variance(&numbers);
    f32::sqrt(v)
}

/// Calculate the variance from of a list of numbers
///
/// # Examples
///
/// ```
/// let arg = vec![1.0,2.0,3.0,4.0, 5.0];
/// let var = easy_stats::variance(&arg);
///
/// assert_eq!(2.5, var);
/// ```
pub fn variance(numbers : &Vec<f32>)-> f32{
    let length = numbers.len();
    let mut v: f32 = 0.0;
    let average = mean(&numbers);

    for num in numbers{
        v=v+ (num - average) * (num - average);
    }
    v / (length-1) as f32
}