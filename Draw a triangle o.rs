fn main ()
{
    // input
    println! ("Enter your number for draw triangle o ");
    let mut x = get_input ();
    
    // the operation & output
    let _n = x;
    for i in 0..x
    {
        x -= 1;
        for _z in 0..i
        {
            print! ("  ");
        }
        for _j in 0..x + 1
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