pub trait ObjectStore {
    fn put(&self, key: &str, value: &[u8]) -> Result<(), String>;
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String>;
    fn delete(&self, key: &str) -> Result<(), String>;
}
