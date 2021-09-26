extern crate meval;

fn main() {
    let initial_value = -2.0;
    let max_iterations = std::env::args().nth(2).expect("Max iterations not set!");
    let equation = std::env::args().nth(3).expect("No equation supplied");
    let derivative = std::env::args().nth(4).expect("No equation derivative supplied");

    let expr: meval::Expr = equation.parse().expect("Unable to parse equation, check your typing!");
    let equation = expr.bind("x").expect("Equation does not contain variable!");

    let expr: meval::Expr = derivative.parse().expect("Unable to parse derivative, check your typing!");
    let derivative = expr.bind("x").expect("Derivative does not contain variable!");

    //let initial_value = initial_value.parse::<f64>().expect("Non number supplied to initial value");
    let max_iterations = max_iterations.parse::<i32>().expect("Non number supplied to max iterations");
    let mut current_value = initial_value;
    let mut next_x = 0.0;
    let mut error = 0.0;


    print!("{:<8}  ", "Iteration");
    print!("{:>+20.18}  ", "Current");
    print!("{:>+20.18}  ", "F(x)");
    print!("{:>+20.18}  ", "F'(x)");
    print!("{:>+20.18}  ", "Delta");
    print!("{:>+20.18}  ", "Next");
    println!("{:>+20.18}", "Error");

    for i in 0..max_iterations {
        let fx = equation(current_value);
        let dx = derivative(current_value);

        let delta = fx/dx;
        next_x = current_value - delta;
        error = (current_value - next_x).abs();

        print!("{:<8} ", i);
        print!("{:>+20.18}  ", current_value);
        print!("{:>+20.18}  ", fx);
        print!("{:>+20.18}  ", dx);
        print!("{:>+20.18}  ", delta);
        print!("{:>+20.18}  ", next_x);
        println!("{:>+20.18}", error);

        current_value = next_x;
    }
}
