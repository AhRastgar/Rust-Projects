fn main ()
{
    // input
    println! ("Enter a first number ");
    let x = get_input ();
    println! ("Enter a second number ");
    let y = get_input ();
    
    // the operation
    let mut sum = 0;
    for i in 0..y
    {
        sum += x;
    }
    
    // output
    println! ("Product = {}", sum);
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}