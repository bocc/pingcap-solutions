pub struct KvStore;

impl KvStore {
    pub fn new() -> KvStore {
        unimplemented!();
    }
    pub fn get(&self, _key: String) -> Option<String> {
        unimplemented!();
    }
    pub fn set(&mut self, _key: String, _value: String) {
        unimplemented!();
    }
    pub fn remove(&mut self, _key: String) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
