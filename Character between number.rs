fn main ()
{
    // subtitle
    println! ("This project is for deploy character between numbers.");
    
    // input
    println! ("Enter a number for deploy character ");
    let mut x = get_input ();
    
    println! ("Enter a symbol for deploy between number ");
    let symbol: String = get_input_str ();
    
     // variables
    let mut stash;
    let mut rev = 0;
    let mut n = x;
    let _m = n;
    let mut stash2;
    let mut _rev2 = 0;
    
    // the operation
    for _i in 0..x
    {
        stash = x % 10;
        x = x / 10;
        rev = rev * 10 + stash;
    }
    
    for _i in 0..n
    {
        stash2 = n % 10;
        n = n / 10;
        
        print! ("{}{}", stash2, symbol);
    }
}

fn get_input_str () -> String
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : String = line.trim().parse().unwrap();
    return number ;
}

fn get_input () -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}