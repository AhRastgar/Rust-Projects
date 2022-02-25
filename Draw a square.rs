fn main ()
{
    // input
    println! ("Enter a number for side of square ");
    let x = get_input ();
    
    // the operation
    for i in 0..x
    {
        for j in 0..x
        {
            print! ("* ");
        }
        print! ("\n");
    }
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}