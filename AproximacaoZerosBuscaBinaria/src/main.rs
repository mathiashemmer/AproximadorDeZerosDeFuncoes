extern crate meval;

fn main() {
    let initial_bound = std::env::args().nth(1).expect("No initial value supplied");
    let final_bound = std::env::args().nth(2).expect("No final value supplied");
    let equation = std::env::args().nth(3).expect("No equation supplied");
    let max_iterations = std::env::args().nth(4).expect("No max iterations supplied");

    let expr: meval::Expr = equation.parse().expect("Unable to parse equation, check your typing!");
    let func = expr.bind("x").expect("Equation does not contain variable!");

    let initial_bound = initial_bound.parse::<f64>().expect("Non number supplied to initial bound");
    let final_bound = final_bound.parse::<f64>().expect("Non number supplied to final bound");
    let max_iterations = max_iterations.parse::<i32>().expect("Non number supplied to max iterations");

    let mut a :f64 = initial_bound;
    let mut b :f64 = final_bound;
    let mut x :f64 = 0.0;
    let mut fa:f64 = 0.0;
    let mut fb:f64 = 0.0;
    let mut fx:f64 = 0.0; 

    for i in 1..(max_iterations+1){
        if ((b-a)/2.0 == 0.0){
            break;
        }

        x = (a + b)/2.0;
        fa = func(a);
        fb = func(b);
        fx = func(x);

        print!("{:<8} ", i);
        print!("{:>12.10}  ", a);
        print!("{:>12.10}  ", x);
        println!("{:>12.10}", b);

        if fa*fx > 0.0{
            a = x;
        }else{
            b = x;
        }
    }
    println!("Aproximation reached at: {}", x);
}
