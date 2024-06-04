//给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请
//
// 你返回所有和为 0 且不重复的三元组。
//
// 注意：答案中不可以包含重复的三元组。
//
//
//
//
//
// 示例 1：
//
// 输入：nums = [-1,0,1,2,-1,-4]
// 输出：[[-1,-1,2],[-1,0,1]]
// 解释：
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
// 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
// 注意，输出的顺序和三元组的顺序并不重要。
// 示例 2：
//
// 输入：nums = [0,1,1]
// 输出：[]
// 解释：唯一可能的三元组和不为 0 。
// 示例 3：
//
// 输入：nums = [0,0,0]
// 输出：[[0,0,0]]
// 解释：唯一可能的三元组和为 0 。

pub struct Solution {}


impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = Vec::new();
        let mut list = nums;
        list.sort();
        for i in 0..list.len() - 2 {
            if i < list.len() - 2 && i != 0 && list[i] == list[i-1] {
                continue;
            }
            let target = - list[i];
            let mut j = i + 1;
            let mut k = list.len() - 1;
            while j < k {
                while j < k && j != i + 1 && list[j] == list[j-1] {
                    j += 1;
                }
                if j < k {
                    let sum = list[j] + list[k];
                    if sum == target {
                        result.push(vec![list[i], list[j], list[k]]);
                        j += 1;
                    } else if sum < target {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }
        }
        result
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test1() {
        let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        println!("{:?}", result);
    }

    #[test]
    pub fn test2() {
        let result = Solution::three_sum(vec![0,0,0]);
        println!("{:?}", result);
    }

    #[test]
    pub fn test3() {
        let result = Solution::three_sum(vec![0,1,1]);
        println!("{:?}", result);
    }

    #[test]
    pub fn test4() {
        let result = Solution::three_sum(vec![0,0,0,0]);
        println!("{:?}", result);
    }
}