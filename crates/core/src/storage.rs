pub trait Storage {
    // The type of the key used for storage, must implement the Address trait.
    type Key: crate::address::Address;

    // The type of the value stored, must implement the SecretKey trait.
    type Value: crate::keypair::SecretKey;

    // Retrieves a value associated with the given key, if it exists.
    fn get(&self, key: Self::Key) -> Option<Self::Value>;

    // Inserts or updates the value associated with the given key.
    fn set(&mut self, key: Self::Key, value: Self::Value);

    // Removes the value associated with the given key.
    fn remove(&mut self, key: Self::Key);

    // Clears all key-value pairs from the storage.
    fn clear(&mut self);

    // Returns an iterator over all key-value pairs in the storage.
    fn iter(&self) -> Box<dyn Iterator<Item = (Self::Key, Self::Value)>>;

    // Checks if the storage contains the given key.
    fn contains_key(&self, key: &Self::Key) -> bool;

    // Returns the number of key-value pairs in the storage.
    fn len(&self) -> usize;

    // Checks if the storage is empty.
    fn is_empty(&self) -> bool;

    // Returns an iterator over all keys in the storage.
    fn keys(&self) -> Box<dyn Iterator<Item = Self::Key>>;

    // Returns an iterator over all values in the storage.
    fn values(&self) -> Box<dyn Iterator<Item = Self::Value>>;
}
