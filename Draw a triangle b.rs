fn main ()
{
    // input
    println! ("Enter a number for draw triangle a ");
    let x = get_input ();
    
    // the operation & output
    for i in 0..x + 1
    {
        for j in 0..i
        {
            print! ("* ");
        }
        print! ("\n");
        
        for l in i..x + 1
        {
            print! ("  ");
        }
    }
    print! ("\n");
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}