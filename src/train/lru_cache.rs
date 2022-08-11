/// simple LRU Cache
/// https://leetcode.com/problems/lru-cache/
/// Doubly-linked list + hashmap, O(1) complexity

struct LRUCache;

impl LRUCache {
    fn new(_capacity: i32) -> Self {
        LRUCache {
            // capacity: capacity as usize,
            // map: HashMap::new(),
            // head: None,
            // tail: None,
        }
    }

    fn get(&mut self, _key: i32) -> i32 {
        0
    }

    fn put(&mut self, _key: i32, _value: i32) {}
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[ignore]
    #[test]
    fn test_case_1() {
        let mut ret: i32;
        let mut lRUCache: LRUCache = LRUCache::new(2);
        lRUCache.put(1, 1); // cache is {1=1}
        lRUCache.put(2, 2); // cache is {1=1, 2=2}
        ret = lRUCache.get(1); // return 1
        assert_eq!(1, ret);

        lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        ret = lRUCache.get(2); // returns -1 (not found)
        assert_eq!(-1, ret);

        lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        ret = lRUCache.get(1); // return -1 (not found)
        assert_eq!(-1, ret);

        ret = lRUCache.get(3); // return 3
        assert_eq!(3, ret);
        ret = lRUCache.get(4); // return 4
        assert_eq!(4, ret);
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let mut ret: i32;
        let mut lRUCache: LRUCache = LRUCache::new(2);
        lRUCache.put(1, 0); // cache is {1=0}
        lRUCache.put(2, 2); // cache is {1=0, 2=2}
        ret = lRUCache.get(1); // return 0, cache is {2=2, 1=0}
        assert_eq!(0, ret);

        lRUCache.put(3, 3); // cache is {1=0, 3=3}
        ret = lRUCache.get(2); // returns -1 (not found)
        assert_eq!(0, ret);

        lRUCache.put(4, 4); // cache is {4=4, 3=3}
        ret = lRUCache.get(1); // return -1 (not found)
        assert_eq!(-1, ret);

        ret = lRUCache.get(3); // return 3
        assert_eq!(3, ret);
        ret = lRUCache.get(4); // return 4
        assert_eq!(4, ret);
    }
}
