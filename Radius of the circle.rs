fn main ()
{
    println! ("Enter a number for radius of the circle ");
    let a = get_input ();
    
    let x: f32 = 2.0;
    
    println! ("Diameter = {}", a * x);
    println! ("printle! = {}", x * 3.14 * a);
    println! ("Area = {}", 3.14 * a * a);
}

fn get_input() -> f32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : f32 = line.trim().parse().unwrap();
    return number ;
}