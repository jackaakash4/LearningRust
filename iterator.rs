 fn main(){
     let needle = 42;
     let heystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

     for item in &heystack{
         if *item == needle {
             println!("{}", item);
         }
     }
 }

