use std::collections::HashMap;
use std::hash::Hash;

fn list_to_map<K, T, M, N>(vec: Vec<T>, m: M, n: N) -> HashMap<K, T>
    where
        K: Eq + Hash,
        M: Fn(&T) -> K,
        N: Fn(T) -> T, // Note that this closure now takes T, not a reference
{
    let mut map = HashMap::new();
    for a in vec {
        let key = m(&a);
        let value = n(a); // a is moved here
        map.insert(key, value);
    }
    map
}

#[derive(Debug)]
struct Person {
    name : String,
    age : i32
}


impl Person {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn age(&self) -> i32 {
        self.age
    }

}

#[test]
fn test() {
    let mut vec: Vec<Person> = Vec::new();
    let tom = Person {
        name: "tom".to_string(),
        age: 13,
    };
    let claude = Person {
        name: "claude".to_string(),
        age: 14,
    };
    vec.push(tom);
    vec.push(claude);
    let map = list_to_map(vec, |a|a.name().to_string(), |a|a);
    println!("map 是 {:?}", map)

}