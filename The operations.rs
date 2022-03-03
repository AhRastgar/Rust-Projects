fn main ()
{
    // input
    println! ("Enter 5 number for operations ");
    let n = get_input ();
    let y = get_input ();
    let z = get_input ();
    let u = get_input ();
    let s = get_input ();
    
    let x: [i32; 5] = [n, y, z, u, s];
    
    // variables
    let mut min = x [0];
    let mut max = x [0];
    let mut sum = 0;
    let mut avg = 0;
    let mut sd = 0;
    let mut st = 0;
    let mut sum2 = 0;
    let mut a = 0;
    
    // the operation
    for i in 0..5
    {
        if x [i] < min
        {
            min = x [i];
        }
        if x [i] > max
        {
            max = x [i];
        }
    }
    
    sum = x [0] + x [1] + x [2] + x [3] + x [4];
    avg = sum / 5;
    
    for j in 0..5
    {
        st = x [j] - avg;
        sum2 = st * st;
        sd = sum2 / 5;
    }
    
    for l in 0..sd
    {
        a = sd / l;
        if a == l
        {
            break;
        }
    }
    
    // output
    println! ("sum = {}", sum);
    println! ("avg = {}", avg);
    println! ("min = {}", min);
    println! ("max = {}", max);
    println!("varians = {}", sd);
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}