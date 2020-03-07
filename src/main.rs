fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    // TODO: Remove
    fn sample_test() {
        assert_eq!(1 + 2, 3);
    }
}
