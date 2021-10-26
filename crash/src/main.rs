fn main() {
    let users = ["jane", "harry", "samuel"];

    let students: [&str; 4] = ["isa", "jane", "harry", "samuel"];

    let gamers: [&str; 4] = ["metro"; 4];

    for user in users.iter() {
        println!("{}", user);
    }

    for student in students.iter() {
        println!("{}", student);
    }

    for gamer in gamers.iter() {
        println!("{}", gamer);
    }

    let sum_arrs = [34, 65];

    println!("The sum of both items are {}", sum_array(sum_arrs));

    fn sum_array(arr: [i32; 2]) -> i32 {
        return arr[0] + arr[1];
    }
}
