fn main ()
{
    println!("This project is for draw a hollow rectangle.");
    
    println! ("Enter a number for length of the rectangle ");
    let length = get_input();
    
    println! ("Enter a number for width of the rectangle ");
    let width = get_input();
    
    for i in 0..length
    {
        print! ("* ");
    }
    print! ("\n");
    
    for j in 0..width - 2
    {
        print! ("*");
        for z in 0..length - 2
        {
            print! ("  ");
        }
        println! (" *");
    }
    
    for h in 0..length
    {
        print! ("* ");
    }
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}