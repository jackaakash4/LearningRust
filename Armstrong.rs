use std::io;

fn main(){
    println!("Enter a number to check armstrong number or not ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to take input");

    let mut num:i32 = match input.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Invalid input");
            return;
        }
    };

    let const_num:i32 = num;
    let mut reminder:i32;
    let mut sum:i32 = 0;
    while num != 0 {
        reminder = num % 10;
        println!("reminder: {}", reminder);
        sum = sum + (reminder * reminder * reminder);
        println!("\n sum: {}", sum);
        num = num/10;
        println!("\n num: {}", num);
    }
    if const_num == sum {
        println!("{} is an armstrong number", const_num);
    }else{
        println!("{} is not an armstrong number", const_num);
    }
}
