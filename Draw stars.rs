fn main ()
{
    // input
    println! ("Enter a number for found golden number ");
    let x = get_input ();
    
    // the operation & output
    for i in 0..x
    {
        println! ("*");
    }
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}