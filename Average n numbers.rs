fn main ()
{
    // input
    println! ("Enter a number for input number ");
    let x = get_input ();
    
    // the operation
    let mut sum = 0;
    for i in 0..x
    {
        println! ("Enter your numbers ");
        let y = get_input ();
        sum += y
    }
    
    // output
    println! ("Average is = {}", sum/x);
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}