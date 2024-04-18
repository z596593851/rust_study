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