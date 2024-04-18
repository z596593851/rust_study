
mod some_thing;
use some_thing::trait_demo;
use trait_demo::Behavior;

fn main() {
    let tom = trait_demo::Person {
        name: String::from("tom"),
        age: 18
    };
    println!("{}", tom.speak());
    println!("{}", tom.eat());
    println!("{}", tom.work());
    println!("====");
    let jerry = trait_demo::Dog {
        name: String::from("jerry"),
        age: 2
    };
    println!("{}", jerry.speak());
    println!("{}", jerry.eat());

}


