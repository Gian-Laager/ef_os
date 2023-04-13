use crate::*;
use macros::os_test;

// #[proc_macro]
// pub fn os_test(item: TokenStream) -> TokenStream {
//     item
// }

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
    println!("all test cases passd");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[os_test]
    fn add() {
        assert_eq!(1 + 1, 2);
    }

    #[os_test]
    fn fail() {
        assert_eq!(1, 2);
    }
}
