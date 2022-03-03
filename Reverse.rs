fn main ()
{
    // subtitle
    println! ("This project is for symmetrical number.");
    
    // input
    println! ("Enter a number for check symmetrical number ");
    let mut x = get_input ();
    
    // variables
    let mut stash;
    let mut rev = 0;
    let n = x;
    
    // the operation
    for _i in 0..x
    {
        stash = x % 10;
        x = x / 10;
        rev = rev * 10 + stash;
    }
    
    // output
    if n == rev
    {
        println! ("This number is a symmetrical number.");
    }
    else
    {
        println! ("This number isn't a symmetrical number.");
    }
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}