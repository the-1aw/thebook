/// For some reason a deranged mind came up with this function.
/// This will increment value of num pointer target by other pointer target.
/// In order to use it, you must make sure both num and other point to valid i32.
unsafe fn unsafe_inc(num: *mut i32, other: *const i32) {
    *num += *other;
}

pub fn safe_inc(num: &mut i32, other: i32) {
    let num_ptr = num as *mut i32;
    let other_ptr = &other as *const i32;
    unsafe {
        unsafe_inc(num_ptr, other_ptr);
    }
}

pub fn raw_pointers(mut num: i32) -> i32 {
    let ptr1 = &num as *const i32;
    let ptr2 = &mut num as *mut i32;
    unsafe {
        *ptr2 += 1;
        *ptr1
    }
}

// c in rust
extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn c_abs(input: i32) -> i32 {
    unsafe { abs(input) }
}

pub static mut STATIC_VARIABLE_COUNTER: i32 = 0;

pub fn add_to_static(num: i32) {
    unsafe {
        STATIC_VARIABLE_COUNTER += num;
    }
}

/// When implementing this trait, you must make sure self + other will not overflow
pub unsafe trait MadIncrementor<T> {
    fn mad_inc(&mut self, other: T);
}

unsafe impl MadIncrementor<i32> for i32 {
    fn mad_inc(&mut self, other: i32) {
        if *self > std::i32::MAX - other {
            panic!("NO NO JOSE!!!");
        }
        *self += other
    }
}

// rust for c
#[no_mangle]
pub extern "C" fn rust_function() {
    println!("Rust in C");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_pointers() {
        let int = 356;
        assert_eq!(raw_pointers(int), int + 1);
    }

    #[test]
    fn test_safe_inc() {
        let mut int = 255;
        safe_inc(&mut int, 1);
        assert_eq!(int, 256)
    }

    #[test]
    fn test_add_to_static() {
        add_to_static(12);
        unsafe {
            assert_eq!(STATIC_VARIABLE_COUNTER, 12);
        }
    }

    #[test]
    fn test_mad_inc() {
        let mut int: i32 = 0;
        int.mad_inc(12);
        assert_eq!(int, 12);
    }

    #[test]
    #[should_panic(expected = "NO NO JOSE!!!")]
    fn test_mad_inc_panic() {
        let mut int: i32 = 14;
        int.mad_inc(std::i32::MAX);
    }
}
