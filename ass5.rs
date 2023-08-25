use std::collections::HashMap as OtherHashMap;

// A trait that defines a method for sorting the elements in a map by their key values.
trait SortByKey {
    fn sort_by_key(&mut self);
}

// A generic HashMap struct that takes two type parameters and implements the SortByKey trait.
struct HashMap<const K: usize, V> {
    map: Box<HashMap<K, V>>,
}

impl<const K: usize, V> HashMap<K, V> {
    // Constructs a new HashMap instance.
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    // Adds a new key-value pair to the map.
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    // Sorts the elements in the map by their key values.
    fn sort_by_key(&mut self) {
        self.map.sort_by(|a, b| a.key().cmp(b.key()));
    }
}

impl<K: Ord, V> SortByKey for HashMap<K, V> {
    fn sort_by_key(&mut self) {
        // This implementation simply calls the "sort_by_key()" method on the underlying HashMap.
        self.map.sort_by_key();
    }
}

fn main() {
    // Create a new HashMap instance.
    let mut map = HashMap::new();

    // Insert some key-value pairs into the map.
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    // Sort the elements in the map by their key values.
    map.sort_by_key();

    // Print the elements in the map.
    for (key, value) in &map {
        println!("key: {}, value: {}", key, value);
    }
}
