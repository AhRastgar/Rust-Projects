fn main ()
{
    // input
    println! ("Enter a number for found golden number ");
    let x = get_input ();
    
    // the operation & output
    println! ("If the number you give is the length, the ratio is = {}", x/1.618);
    println! ("if the number you give is the width, the ratio is = {}", x*1.618);
}

fn get_input() -> f32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : f32 = line.trim().parse().unwrap();
    return number ;
}