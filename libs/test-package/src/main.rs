fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_test() {
        dbg!(std::env::current_dir().unwrap());
        panic!();
    }
}
