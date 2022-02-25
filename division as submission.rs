fn main ()
{
    // input
    println! ("Enter a first number ");
    let x = get_input ();
    println! ("Enter a second number ");
    let y = get_input ();
    
    // the operation
    let mut m = x;
    let mut a = 0;
    
    for i in 0..y
    {
        if m >= y
        {
            a += 1;
            m -= y;
        }
        else
        {
            break;
        }
    }
    
    // output
    println! ("Outside the split part = {}", a);
    println! ("Division remaining = {}", m);
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}