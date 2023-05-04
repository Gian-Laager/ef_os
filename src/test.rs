use crate::*;
use macros::os_test;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    print!("running {} test", tests.len());
    if tests.len() > 1 {
        println!("s");
    } else {
        println!()
    }
    for test in tests.iter() {
        test();
    }
    println!("\n\x1b[42mall test cases passed\x1b[0m");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[os_test]
    fn test_system_check() {
        assert!(true);
        assert_eq!(1, 1);
        assert_ne!(1, 2);
    }
}
