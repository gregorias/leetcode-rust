// https://leetcode.com/problems/two-sum/

mod two_sum {
    use std::collections::HashMap;

    #[allow(unused)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_idx_hashmap : HashMap<i32, usize> = HashMap::new();


        for i in 0..nums.len() {
            let complement_idx = num_to_idx_hashmap.get(&(target - nums[i]));
            if let Some(val) = complement_idx {
                return vec![*val as i32, i as i32];
            }
            num_to_idx_hashmap.insert(nums[i], i);
        }
        vec![]
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_two_sum_1() {
            let nums = vec![2, 7, 11, 15];
            let target = 9;
            let expected = vec![0, 1];
            assert_eq!(two_sum(nums, target), expected);
        }

        #[test]
        fn test_two_sum_2() {
            let nums = vec![3, 2, 4];
            let target = 6;
            let expected: Vec<i32> = vec![1, 2];
            assert_eq!(two_sum(nums, target), expected);
        }

        #[test]
        fn test_two_sum_3() {
            let nums = vec![3, 3];
            let target = 6;
            let expected: Vec<i32> = vec![0, 1];
            assert_eq!(two_sum(nums, target), expected);
        }
    }
}
