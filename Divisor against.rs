fn main ()
{
    // input
    println! ("Enter a number for check divisor against ");
    let x = get_input ();
    
    // the operation & output
    let mut num = x;
    let mut sum = 0;
    
    for i in 1..x + 1
    {
        if num % i == 0
        {
            println! ("Divide by your number is = {}", i);
        }
    }
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}