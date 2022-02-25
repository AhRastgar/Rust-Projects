fn main ()
{
    // input
    println! ("Enter a number, check it for perfect number ");
    let x = get_input ();
    
    // the operation
    let mut n = x;
    let mut sum = 0;
    
    for i in 1..x
    {
        if n % i == 0
        {
            sum += i;
        }
    }
    
    // output
    if sum == x
    {
        println! ("Yes");
    }
    else
    {
        println! ("No");
    }
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}