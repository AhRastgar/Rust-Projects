fn main ()
{
    println!("This project is for draw a hollow triangle.");
    
    println! ("Enter a number for length of the triangle ");
    let length = get_input();
    
    println! ("Enter u or d for direction (u: up , d: down)");
    let dir = getInput();
    
    let u;
    let d;
    
    if dir == u
    {
        for i in 1..length - 1
        {
            print! ("*");
            if i > 1
            {
                for j in 1..i
                {
                    print! ("  ");
                }
            }
            if i > 0
            {
                println! ("*");
            }
            else
            {
                print! ("\n");
            }
        }
        
        for h in 0..length
        {
            print! ("* ");
        }
    }
    
    if dir == d
    {
        for z in 0..length
        {
            print! ("* ");
        }
        print! ("\n");
        
        for r in length--
        {
            print! ("*");
            if r < length + 2
            {
                for g in 0..r - 1
                {
                    print! ("  ");
                }
                println! ("*");
            }
            else
            {
                print! ("\n");
            }
        }
    }
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}

fn getInput() -> u32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : u32 = line.trim().parse().unwrap();
    return number ;
}