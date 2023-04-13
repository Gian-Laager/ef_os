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
    for (i, test) in tests.iter().enumerate() {
        test();
    }
    println!("all test cases passed");
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
