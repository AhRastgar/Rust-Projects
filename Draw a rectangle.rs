fn main ()
{
    // input
    println! ("Enter a number for length of rectangle ");
    let x = get_input ();
    println! ("Enter a number for width of rectangle ");
    let y = get_input ();
    
    // the operation
    for i in 0..y
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