fn main ()
{
    // input
    println! ("Enter a number ");
    let x = get_input ();
    println! ("Enter a number for power ");
    let y = get_input ();
    let mut a = 1;
    
    // the operation
    for mut _i in 0..y
    {
        a *= x;
    }
    println! ("x pow y = {}", a);
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}