pub mod eigr {
    #[path = "eigr.spawn.rs"]
    pub mod spawn {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
