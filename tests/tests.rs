/*
#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use super::*;
    #[test]
    fn test_display_a_single_dot() {
        let mut image = [[0; 5]; 5]; 
        display_a_single_dot(&mut image); 
        panic_assert_eq!(image[2][2], 9);
    }
}
*/