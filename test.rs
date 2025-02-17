// enum Option<T> {
//     Some(T),
//     None,
// }

fn main(){
    let some_number = Some(5);
    let some_string = Some("строковый литерал");
    let absent_number: Option<i32> = None;
    println!("sume_number: {}", some_number);
    // println!("sume_number: {}, some_string: {}, absent_number: {}", some_number, some_string, absent_number);
}