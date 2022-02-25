fn main ()
{
    // input
    println! ("Enter a chorm ");
    let a = get_input ();
    println! ("Enter a Opposite to the chord ");
    let b = get_input ();
    println! ("Enter a Adjacent to the chord ");
    let c = get_input ();
    
    // the operation & output
    if a > b + c
    {
        print! ("This is not a triangle.");
    }
    else
    {
        print! ("This is a triangle.");
    }
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}