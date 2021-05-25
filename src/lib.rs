mod some_fns;

#[cfg(test)]
mod tests {
    use crate::some_fns::add;
    #[test]
    fn it_works() {
        assert_eq!(add(2, 3), 5);
    }
}
