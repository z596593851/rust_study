
pub struct Person {
    pub name:String,
    pub age:i32
}

pub struct Dog {
    pub name:String,
    pub age:i32
}

pub trait Behavior {
    fn speak(&self) -> String;

    fn eat(&self) -> String {
        String::from("eat something")
    }
}

impl Behavior for Person {
    fn speak(&self) -> String {
        let name = &self.name;
        let age = &self.age;
        format!("name is {}, age is {}", name, age)
    }
}

impl Behavior for Dog {
    fn speak(&self) -> String {
        format!("wang wang, name is {}, age is {}", self.name, self.age)
    }
}

impl Person {
    pub fn work(&self) -> String {
        String::from("i work")
    }
}

#[test]
fn test() {
    let tom = Person {
        name: String::from("tom"),
        age: 18
    };
    println!("{}", tom.speak());
    println!("{}", tom.eat());
    println!("{}", tom.work());
    println!("====");
    let jerry = Dog {
        name: String::from("jerry"),
        age: 2
    };
    println!("{}", jerry.speak());
    println!("{}", jerry.eat());
}