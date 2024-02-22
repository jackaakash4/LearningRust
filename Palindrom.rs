use std::io;

fn main(){
    println!("Enter a number to check palindrom \n");
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut num: i32 = match input.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Invalid input! Please enter valid number");
            return;
        }
    };

    println!("You have entered: {}", num);
    let cnum:i32 = num;
    let mut reverse:i32 = 0;
    let mut reminder:i32;
    while num != 0{
        reminder = num%10;
        reverse = reverse * 10 + reminder;
        num /= 10;
    }
    if cnum == reverse{
        println!("{} is palindrom", cnum);
    }else{
        println!("{} is not palindrom", cnum);
    }

}
    
