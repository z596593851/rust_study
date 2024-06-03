// 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
//
// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
//
//
//
// 示例 1：
//
// 输入：nums = [100,4,200,1,3,2]
// 输出：4
// 解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
// 示例 2：
//
// 输入：nums = [0,3,7,2,5,8,4,6,0,1]
// 输出：9
//
//
// 提示：
//
// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109

// max_cons(小于等于它的都连续), min_not_cons(大于它的都不连续)

use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut count = 1;
        let mut list = nums;
        list.sort();
        let mut count_temp = 1;
        for index in 0..list.len() {
            let index_i32 = index as i32;
            if index_i32 != 0 {
               if list[index_i32] ==  list[index_i32 - 1] + 1 {
                   count_temp += 1;
                   count = cmp::max(count, count_temp)
               } else {
                   count_temp = 1;
               }
            }
        }
        count
    }
}


pub fn test() {

}