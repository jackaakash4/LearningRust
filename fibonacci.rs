fn main(){
    let mut t1:i32 = 0;
    let mut t2:i32 = 1;
    let mut next_term:i32 = t1 + t2;
    println!("Fibonacci series: \n{} \n{} ", t1, t2);

    let i:i32 = 3;

    for _i in i..10{
        println!("{}", next_term); 
        t1 = t2;
        t2 = next_term;
        next_term = t1 + t2;
       
    }
}

