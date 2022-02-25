fn main ()
{
    // input
    println! ("Enter a number radius ");
    let x = get_input ();
    
    // the operation
    let mut a = 2.0 * 3.14 * x;
    let mut p = a / x;
    
    // output
    println! ("The expansion of pi number is = {}", p);
}

fn get_input() -> f32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : f32 = line.trim().parse().unwrap();
    return number ;
}