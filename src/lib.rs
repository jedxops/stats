// Copyright © 2019 Jeff Austin
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Jeff Austin wrote the following function implementations.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;

/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```

pub fn mean(nums: &[f64]) -> Option<f64> {
    //    unimplemented!("no mean yet")
    if nums.is_empty() {
        return Some(0.0);
    }
    let mut sum: f64 = 0.0;
    let len = nums.len() as f64;
    for i in 0..nums.len() {
        sum += nums[i];
    }
    let mean: f64 = sum / len;
    Some(mean)
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```

pub fn stddev(nums: &[f64]) -> Option<f64> {
    //    unimplemented!("no stddev yet")

    // ***********************************
    // based off the formula from: https://www.thoughtco.com/calculate-a-sample-standard-deviation-3126345
    // Step 1 --Calculate mean
    // Step 2 --Store the result of the mean subtracted from each element of the input list and square the result
    // Step 3 --Add all of the previous steps `results` together
    // Step 4 --Divide the previous sum by the total number of items in the input list minus 1
    // Step 5 --Return the result in the form of an option f64 type
    // ***********************************
    if nums.is_empty() {
        return None;
    }
    let result = mean(nums);
    let mean = match result {
        Some(x) => x,
        None => 0.0,
    };
    let mut xs: Vec<f64> = Vec::new();
    let mut sum: f64 = 0.0;
    for i in 0..nums.len() {
        xs.push((nums[i] - mean) * (nums[i] - mean)); // subtracting the mean from every item in the provided list and squaring the result
    }
    for i in 0..xs.len() {
        sum += xs[i];
    }
    let stddev: f64 = sum / (nums.len() - 1) as f64;
    Some(stddev.sqrt())
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    //    unimplemented!("no median yet")
    if nums.is_empty() {
        return None;
    }
    // if the length of the input is even (hard case)
    if nums.len() % 2 == 0 {
        let j: usize = nums.len() / 2;
        if j < (nums.len() - 1 - j) {
            return Some(nums[j]);
        } else {
            return Some(nums[j - 1]);
        }
    }
    // if the length of the input is odd
    else {
        for i in 0..nums.len() {
            if (i) == (nums.len() - (i + 1)) {
                return Some(nums[i].abs());
            }
        }
    }
    None
}

/*    // if the length of the input is even (hard case)
if nums.len() % 2 == 0 {
    for i in 0..nums.len() {
        if (i + 1) == ((nums.len() -1) - i) {
            return Some(((nums[i].abs() + nums[i+1].abs())/2.0));
        }
    }
}*/

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
pub fn l2(nums: &[f64]) -> Option<f64> {
    //    unimplemented!("no l2 yet")
    let mut sum: f64 = 0.0;
    let mut v: Vec<f64> = Vec::new();
    for i in 0..nums.len() {
        v.push(nums[i] * nums[i]);
    }
    for i in 0..v.len() {
        sum += v[i];
    }
    Some(sum.sqrt())
}

//unit test for the mean function
#[test]
pub fn test_neg() {
    assert_eq!(
        Some(4.8),
        mean(&[-25.8, -18.8, -2.0, 0.0, 1.0, 1.8, 18.0, 25.8, 56.0, -8.0])
    )
}
//unit test for the stddev function
#[test]
pub fn test_large_list() {
    assert_eq!(
        Some(50.02867767334641),
        stddev(&[-25.8, -18.8, -2.0, 0.0, 1.0, 1.8, 18.0, 25.8, 56.0, -8.0, -90.0, 90.9, 100.9])
    )
}
//unit test for the median function
#[test]
pub fn test_obscure() {
    assert_eq!(
        Some(-8.0),
        median(&[
            18.0, -25.8, 56.0, -8.0, -90.0, 90.9, 25.8, 100.9, -56.0, -100.89, -100.9, 100.89
        ])
    )
}
//unit test for the l2 function
#[test]
pub fn test_negative_vector() {
    assert_eq!(Some(999.4768631639254), l2(&[-25.0, -18.0, -2.0, -999.0]))
}
