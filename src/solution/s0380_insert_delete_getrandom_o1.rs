/**
 * [380] Insert Delete GetRandom O(1)
 *
 * Implement the RandomizedSet class:
 *
 * 	RandomizedSet() Initializes the RandomizedSet object.
 * 	bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
 * 	bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
 * 	int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
 *
 * You must implement the functions of the class such that each function works in average O(1) time complexity.
 *  
 * Example 1:
 *
 * Input
 * ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
 * [[], [1], [2], [2], [], [1], [2], []]
 * Output
 * [null, true, false, true, 2, true, false, 2]
 * Explanation
 * RandomizedSet randomizedSet = new RandomizedSet();
 * randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
 * randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
 * randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
 * randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
 * randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
 * randomizedSet.insert(2); // 2 was already in the set, so return false.
 * randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= val <= 2^31 - 1
 * 	At most 2 * 10^5 calls will be made to insert, remove, and getRandom.
 * 	There will be at least one element in the data structure when getRandom is called.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-delete-getrandom-o1/
// discuss: https://leetcode.com/problems/insert-delete-getrandom-o1/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use rand;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
struct RandomizedSet {
    elems: Vec<i32>,
    indices: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            elems: Vec::new(),
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        // if self.indices.contains_key(&val) {
        //     false
        // } else {
        //     self.indices.insert(val, self.elems.len() as i32);
        //     self.elems.push(val);
        //     true
        // }

        if let Entry::Vacant(e) = self.indices.entry(val) {
            e.insert(self.elems.len() as i32);
            self.elems.push(val);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.indices.get_mut(&val) {
            None => false,
            Some(&mut index) => {
                // vec 的最后一个元素
                let last_elem = self.elems[self.elems.len() - 1];

                // 如果 remove 的不是 vec 的最后一个元素，需要执行移动 & 更新
                if val != last_elem {
                    self.elems[index as usize] = last_elem;
                    self.indices.insert(last_elem, index);
                }

                self.indices.remove_entry(&val);
                self.elems.pop();
                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        let random_index = rand::random::<usize>() % self.elems.len();
        self.elems[random_index]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_380() {
        let mut set = RandomizedSet::new();

        assert_eq!(true, set.insert(1));
        println!("1111{:?}", set);
        assert_eq!(false, set.remove(2));
        println!("2222{:?}", set);
        assert_eq!(true, set.insert(2));
        println!("3333{:?}", set);

        assert_eq!(2, set.get_random());
    }
}
