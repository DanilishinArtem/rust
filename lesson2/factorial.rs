fn factofial(x: i64) -> i64 
{
    if x == 0
    {
        1
    }
    else
    {
        x * factofial(x - 1)
    }
}

fn main() {
    println!("Factorial of 5 is {}", factofial(5));
}