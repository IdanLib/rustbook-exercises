pub fn add_two(a: i32) -> i32 {
    return internal_adder(a, 2);
}

fn internal_adder(a: i32, b: i32) -> i32 {
    return a + b; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
