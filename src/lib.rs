#[macro_export]
macro_rules! map {
    ($([$k:expr]: $v:expr),* $(,)?) => {
        {
            let mut temp_vec = std::collections::HashMap::new();
            $(
                temp_vec.insert($k, $v);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        {
            let map = map! {
                [1]: "one",
                [2]: "two",
                [3]: "three",
            };
            assert_eq!(map.get(&1), Some(&"one"));
            assert_eq!(map.get(&2), Some(&"two"));
            assert_eq!(map.get(&3), Some(&"three"));
        }

        {
            let map = map! {
                ["first"]: 1,
                ["second"]: 2,
                ["third"]: 3,
            };
            assert_eq!(map.get(&"first"), Some(&1));
            assert_eq!(map.get(&"second"), Some(&2));
            assert_eq!(map.get(&"third"), Some(&3));
        }
    }

    #[test]
    fn test_nested_map() {
        let map = map! {
            [1]: map! {
                [1]: "one",
                [2]: "two",
                [3]: "three",
            },
            [2]: map! {
                [1]: "one",
                [2]: "two",
                [3]: "three",
            },
            [3]: map! {
                [1]: "one",
                [2]: "two",
                [3]: "three",
            },
        };
        assert_eq!(map.get(&1).unwrap().get(&1), Some(&"one"));
        assert_eq!(map.get(&1).unwrap().get(&2), Some(&"two"));
        assert_eq!(map.get(&1).unwrap().get(&3), Some(&"three"));
        assert_eq!(map.get(&2).unwrap().get(&1), Some(&"one"));
        assert_eq!(map.get(&2).unwrap().get(&2), Some(&"two"));
        assert_eq!(map.get(&2).unwrap().get(&3), Some(&"three"));
        assert_eq!(map.get(&3).unwrap().get(&1), Some(&"one"));
        assert_eq!(map.get(&3).unwrap().get(&2), Some(&"two"));
        assert_eq!(map.get(&3).unwrap().get(&3), Some(&"three"));
    }
}
