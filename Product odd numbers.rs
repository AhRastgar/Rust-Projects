fn main ()
{
    // input
    println! ("Enter a first number ");
    let x = get_input ();
    println! ("Enter a second number ");
    let y = get_input ();
    
    // the operation
    let mut prd = 1;
    for i in x..y
    {
        if i % 2 == 1
        {
            prd *= i;
        }
    }
    // output
    println! ("product odd numbers = {}", prd);
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}