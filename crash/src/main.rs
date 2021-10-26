fn main(){
let users = ["jane", "harry", "samuel"];

let students : [&str; 4] = ["isa", "jane", "harry", "samuel"];

let gamers : [&str; 4] = ["metro"; 4];

for user in users.iter(){
    println!("{}", user);
}

for student in students.iter(){
    println!("{}", student);
}

for gamer in gamers.iter(){
    println!("{}", gamer);
}

}
