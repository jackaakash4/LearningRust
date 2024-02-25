fn main(){
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}\n", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}\n", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}\n", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}\n", three, thirty, three_hundred);
}

