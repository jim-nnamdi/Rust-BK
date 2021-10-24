pub struct Dog {
    name: String,
    age: u32,
    owner: String 
}

impl ToString for Dog{
    fn to_string(&self) -> String{
        return format!("{} is {} and is owned by {}", self.name, self.age, self.owner);
    }
}

fn main(){
    let dog = Dog{name: "Frodo".to_string(), age:3, owner:"owens".to_string()};
    println!("{}", dog.to_string());
}