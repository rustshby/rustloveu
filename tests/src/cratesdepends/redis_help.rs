#[cfg(test)]
mod tests {
    extern crate cratesdepends;
    use cratesdepends::redis_help;

    #[test]
    fn it_fetch_an_integer() {
        let value = redis_help::fetch_an_integer();
        println!("{:?}", value);
        assert_eq!(value, Ok(42));
    }
}
