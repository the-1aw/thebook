//! # Bryan's Rust Standard Library
//!
//! This library is a collection of utilities to make common taks more
//! convenient.

/// Add two numbers together.
///
/// # Examples
///
/// ```
/// let left = 1;
/// let right = 2;
/// let sum = brstd::add(left, right);
///
/// assert_eq!(sum, 3);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
