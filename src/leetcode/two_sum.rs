// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
//
// 你可以按任意顺序返回答案。

use std::collections::HashMap;
pub struct Solution {
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:HashMap<i32,i32> = HashMap::new();
        for (index, &num) in nums.iter().enumerate() {
            let x = target - num;
            if let Some(&value_index) = map.get(&x) {
                return vec![index as i32, value_index];
            }
            map.insert(num, index as i32);
        }
        vec![]
    }
}

#[test]
fn test() {
    let result = Solution::two_sum(vec![1, 2, 3, 4], 7);
    println!("{:?}", result);
}