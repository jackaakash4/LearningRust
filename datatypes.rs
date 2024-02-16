fn main(){
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let is_rust_fun: bool = true;
    let letter: char = 'A';
    
    //scaler  type
    println!("Scaler Types:\n");
    println!("Integer:{}", integer);
    println!("Float:{}", float);
    if is_rust_fun{
        println!("Rust is fun");
    }else{
        println!("Rust is not fun");
    }
    println!("Letter: {}", letter);
    
    //compound type
    let tuple: (i32, f64, char) = (10, 3.14, 'B');
    let array: [i32;5] = [1, 2, 3, 4, 5];

    println!("Compound type\n");
    println!("Tuple: {:?}\n", tuple);
    println!("Array: {:?}\n", array);
}


