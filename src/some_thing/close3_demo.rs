use std::collections::HashMap;
use std::hash::Hash;

// 注意生命周期标注 'a，这表明map中的键和值的生命周期至少与输入的Vec中的元素的生命周期一样长
fn list_to_map<K, V, T, M, N>(vec: Vec<&T>, m: M, n: N) -> HashMap<K, &V>
    where
        K: Eq + Hash,
        M: Fn(&T) -> K,
        N: Fn(&T) -> &V,
{
    let mut map = HashMap::new();
    for &a in vec.iter() {
        let key = m(a);
        let value = n(a);
        map.insert(key, value);
    }
    map
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn name(&self) -> &String {
        &self.name
    }

    // 注意这里返回的是对整个Person的引用，这可能在函数签名中需要
    pub fn self_ref(&self) -> &Self {
        self
    }
}

#[test]
fn test() {
    let tom = Person {
        name: "Tom".to_string(),
        age: 30,
    };
    let jerry = Person {
        name: "Jerry".to_string(),
        age: 28,
    };

    let persons = vec![&tom, &jerry];
    // 这里调整调用以适配新的设计，确保我们保存的是对Person的引用
    let person_map = list_to_map(persons, |p| p.name.clone(), |p| p.self_ref());

    // 由于person_map现在存储的是引用，它的使用范围需要在原Person对象存在的范围之内
    // 这里我们可以继维持对tom和jerry的访问权
    println!("映射包含: {:?}", person_map);

    // tom和jerry对象在此处依旧可用
}