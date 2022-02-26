fn main ()
{
    // input
    println! ("Enter a number for draw a triangle z ");
    let mut x = get_input ();
    let mut n = x;
    
    // the operation & output
    for i in 0..x
    {
        for j in 0..x
        {
            print! ("* ");
        }
        x -= 1;
        
        if x == 0
        {
            break;
        }
        print! ("\n");
    }
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}