pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(4, add_two(2));
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}
