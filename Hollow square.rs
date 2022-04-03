fn main ()
{
    println!("This project is for draw a hollow square.");
    let num = get_input();
    
    for i in 0..num
    {
        print! ("* ");
    }
    print! ("\n");
    
    for j in 0..num - 2
    {
        print! ("*");
        for h in 0..2 * num - 3
        {
            print! (" ");
        }
        println! ("*");
    }
    
    for z in 0..num
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