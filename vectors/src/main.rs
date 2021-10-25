
// vectors are resizable arrays in rust.

fn dump(arr: &[i32]){
    println!("array is {:?}", arr);
}

fn main(){
    let mut vector = Vec::new();
    vector.push(10);
    vector.push(20);
    vector.push(30);

    dump(&vector);

    let first_ele = vector[0];
    let may_first = vector.get(0);

    println!("{}", first_ele);
    println!("{:?}", may_first);

}