
use std::io;
const MAX: usize = 187; // over this value Fibonacci number is above u128 capacity --- TO DO: work with big big numbers


fn main() {
    println!("Fibonacci");
    println!(" Please type the term of Fibonacci sequence you want to know (under {MAX}):");

    let mut order: String = String::new();

    io::stdin()
        .read_line(&mut order)
        .expect("failed to read the order");

    
    
    let mut f: [u128; MAX]  = [0; MAX];

    match order.trim().parse::<usize>() {
        Ok(x) => {
            if x < MAX {
                println!("Fibonacci({x}) = {}", fibonacci(x, &mut f))
            } else {
                println!("Please input a positive whole number under {MAX}")
            }
        },
        Err(_) => println!("Please input a positive whole number under {MAX}"),
    }
}

fn fibonacci (n: usize, f: &mut [u128; MAX]) -> u128 
{ 
    if n == 0 {
         return 0;
    }

    if n == 1 || n == 2 {
         f[n] = 1;
         return f[n];
    }
   
    if f[n] != 0 {
        return f[n];
    }
    
    let k = if n&1 == 1 { (n+1)/2 } else { n/2 };
    
    if n&1 == 1  { 
        f[n] = fibonacci(k-1,f)*fibonacci(k-1,f) + fibonacci(k,f)*fibonacci(k,f);
    } else {
        f[n] = (2*fibonacci(k-1,f)+fibonacci(k,f))* fibonacci(k,f);
    }

    return f[n];

}
