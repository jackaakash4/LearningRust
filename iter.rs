//Using iter() method to return an iterator

fn main(){
    let needle = 32;
    let heystack = [1, 2, 3, 4, 5, 66, 77, 88, 23];

    for item in heystack.iter(){
        if *item == needle{
            println!("{}", item);
        }
    }
}

