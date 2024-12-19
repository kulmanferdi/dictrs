# dictrs

A blazing fast dictionary written in rust.

This repository contains a custom implementation of a dictionary (key-value store) using Rust's `VecDeque` collection. The implementation supports operations such as insertion, deletion, retrieval, and checking for the existence of keys.

---

## Features

- **Key-Value Storage**: Stores items with unique keys.
- **Insert**: Adds or updates key-value pairs.
- **Delete**: Removes items by their keys.
- **Retrieve**: Fetches the value associated with a given key.
- **Count**: Counts the number of key-value pairs in the dictionary.
- **Empty**: Clears all items from the dictionary.
- **Contains**: Checks if a specific key exists.

---

## Structure

### `Dictionary<K, V>`
A generic dictionary type that holds key-value pairs using a `VecDeque`.

#### Methods

- **`new()`**  
  Creates a new empty dictionary.  
  ```rust
  let mut dict: Dictionary<String, i32> = Dictionary::new();
  ```
  
- **`empty(&mut self)`**  
  Clears all items in the dictionary.
  ```rust
   dict.empty();
  ```

- **`count(&self) -> usize`**  
  Returns the number of items in the dictionary.
  ```rust
   let item_count = dict.count();
  ```

- **`contains(&self, key: &K) -> bool`**  
  Checks if the dictionary contains a specific key.
  ```rust
  let exists = dict.contains(&"key".to_string());
  ```

 - **`insert(&mut self, key: K, value: V)`**  
   Adds a new key-value pair or updates an existing key's value.
   ```rust
   dict.insert("key".to_string(), 42);
   ```

 - **`delete(&mut self, key: &K) -> Result<(), InvalidKeyException>`**  
   Removes a key-value pair by its key. Returns an error if the key doesn't exist.
   ```rust
   match dict.delete(&"key".to_string()) {
    Ok(_) => println!("Key deleted."),
    Err(_) => println!("Key not found."),
   }
   ```

 - **`get(&mut self, key: &K) -> Result<V, InvalidKeyException>`**  
   Retrieves the value associated with a key. Returns an error if the key doesn't exist.

   ```rust
   match dict.get(&"key".to_string()) {
    Ok(value) => println!("Value: {}", value),
    Err(_) => println!("Key not found."),
   }
   ```


### `Item<K, V>`
Represents a key-value pair stored in the dictionary.

### Fields
- `key: K`
  The key of the item.

- `*value: V`
  The value associated with the key.

## Error Handling
`InvalidKeyException`
An exception type that is returned when operations such as get or delete are performed with a non-existent key.

## Example Usage
 ```rust
 use std::collections::VecDeque;
 use my_crate::Dictionary;

 fn main() {
    let mut dict = Dictionary::new();
    dict.insert("name".to_string(), "Alice");
    dict.insert("age".to_string(), 30);

    println!("Item count: {}", dict.count());
    
    if dict.contains(&"name".to_string()) {
        println!("Name exists in dictionary.");
    }

    match dict.get(&"name".to_string()) {
        Ok(value) => println!("Name: {}", value),
        Err(_) => println!("Name not found."),
    }

    dict.delete(&"name".to_string()).unwrap_or_else(|_| println!("Key not found."));
 }
   ```

## Notes
Keys must implement the PartialEq and Debug traits to enable comparisons and debugging.
Values are stored in a VecDeque to allow efficient insertion and deletion.

## License
This project is open-source and available under the MIT license. See the LICENSE file for details.
