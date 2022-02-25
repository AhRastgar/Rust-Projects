fn main ()
{
    // input
    println! ("Enter a number ");
    let a = get_input ();
    
    // the operation
    println! ("Power 1 = {}", a);
    println! ("Power 2 = {}", a * a);
    println! ("Power 3 = {}", a * a * a);
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}