# Rust easy stats

easy_stats is a library which contains functions that make it easy to perform basic descriptive statistics.


## Usage
Calculate the mean from a list of numbers:

```rust
use easy_stats;
let arg = vec![1.0,2.0,3.0,4.0, 5.0];
let average = easy_stats::mean(&arg);
assert_eq!(3.0, average);
```


## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.
