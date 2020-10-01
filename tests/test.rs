#[macro_use]
extern crate rift_lib;

pub use std::collections::HashMap;

pub fn even(x: i32) -> bool {
    x % 2 == 0
}

pub fn odd(x: i32) -> bool {
    x % 2 != 0
}

#[cfg(test)]
mod tests {
    use super::{even, odd, HashMap};
    #[test]
    fn test_hello_macro() {
        assert_eq!(hello!("John"), "Hello, John");
    }

    #[test]
    fn test_comp_macro() {
        let list_even = comp! (x for x in [1;11], even);
        let vec_list_even = [2, 4, 6, 8, 10];
        assert_eq!(list_even, vec_list_even);

        let list_odd = comp! (x for x in [1;11], odd);
        let vec_list_odd = [1, 3, 5, 7, 9];
        assert_eq!(list_odd, vec_list_odd);
    }

    #[test]
    fn test_dict_macro() {
        let dict = dict!("One" => 1, "Two" => 2, "Three" => 3);
        let mut hash_map = HashMap::<&str, i32>::new();
        hash_map.insert("One", 1);
        hash_map.insert("Two", 2);
        hash_map.insert("Three", 3);

        assert_eq!(dict, hash_map);
    }
}
