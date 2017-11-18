#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    pub fn block_test() {
        for x in 1..10 {
            println!("UNIMPLEMENTED");
        }
    } 
}
