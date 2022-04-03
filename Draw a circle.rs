use turtle::Turtle;

fn main ()
{
    println!("This project is for draw a circle.");
    
    println! ("Enter a number for radius of circle ");
    let radius = get_input();
    let mut turtle = Turtle::new();
    
    for i in 0..360
    {
        turtle.forward (radius);
        turtle.right (1.0);
    }
}

fn get_input() -> f32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : f32 = line.trim().parse().unwrap();
    return number ;
}