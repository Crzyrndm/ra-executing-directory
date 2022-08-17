fn main() {
    dbg!(std::env::current_dir().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_test() {
        dbg!(std::env::current_dir().unwrap());
        panic!();
    }
}
