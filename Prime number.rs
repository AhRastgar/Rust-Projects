fn main ()
{
    println!("This project is for check the prime number.");
    
    println! ("Enter a number for check a prime number or not ");
    let num = get_input();
    let mut check = 0;
    
    for i in 2..num
    {
        if num % i == 0
        {
            check = 1;
        }
    }
    
    if check == 0
    {
        print! ("Yes");
    }
    else
    {
        print! ("No");
    }
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}