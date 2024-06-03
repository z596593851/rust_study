// 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
//
// 字母异位词 是由重新排列源单词的所有字母得到的一个新单词。
//
//
//
// 示例 1:
//
// 输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
// 输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
// 示例 2:
//
// 输入: strs = [""]
// 输出: [[""]]
// 示例 3:
//
// 输入: strs = ["a"]
// 输出: [["a"]]
//
//
// 提示：
//
// 1 <= strs.length <= 104
// 0 <= strs[i].length <= 100
// strs[i] 仅包含小写字母

use std::collections::HashMap;

pub struct Solution {
}

const BASE : i32 = 97;

impl Solution {

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map:HashMap<String,Vec<String>> = HashMap::new();
        // //方法1
        // for str in strs {
        //     let str_encode = Self::encode(&str);
        //     map.entry(str_encode).or_insert_with(Vec::new).push(str);
        // }

        // //方法2
        // for str in strs {
        //     let mut chars: Vec<char> = str.chars().collect();
        //     chars.sort_unstable();
        //     let sorted_str: String = chars.into_iter().collect();
        //
        //     map.entry(sorted_str).or_insert_with(Vec::new).push(str);
        // }

        // 方法3
        for x in strs.iter() {
            // 如果这里的x写作&x，会报错：move occurs because `x` has type `String`, which does not implement the `Copy` trait
            let mut a = x.as_bytes().to_vec();
            a.sort();
            let a = String::from_utf8(a).unwrap();
            map.entry(a).or_insert_with(Vec::new).push(x.clone());
            // match map.get_mut(&a) {
            //     Some(vec) => {
            //         vec.push(x.clone());
            //     }
            //     None => {
            //         map.insert(a,vec![x.clone()]);
            //     }
            // }
        }
        map.into_values().collect()

    }

    pub fn encode(str: &str) -> String {
        let mut array = [0;26];
        for c in str.chars() {
            let c_i32 = c as i32;
            let index : usize = (c_i32 - BASE) as usize;
            array[index] += 1;
        }
        let mut result = String::new();
        for (index, &count) in array.iter().enumerate() {
            if count != 0 {
                // let index_u8 = index as u8;
                let index_u8 = index as u8 + b'a';
                let c = index_u8 as char;
                //a2b7c3
                result.push(c);
                result.push_str(&count.to_string())
            }
        }
        result
    }

}


#[test]
pub fn test() {
    let result = Solution::group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]);
    println!("{:?}",result);

}