fn main() {

    let a:u8 = 1 ;
    let b:u8 = 0 ;
    
    let mut x:u8;
    

//    x = !(!a);
//    println!("X = !(!A)");
//    println!("0b{:08b}",x);

    x = !(!(a & b));
    println!("X = (!(!(a & b))");
    println!("0b{:08b}",x);
    
//   x = !(!(a | b));
//    println!("X = !(!(a | b))");
//    println!("0b{:08b}",x);
    
    
 //   x = !(a & b);
 //   println!("X = !(a & b)");
 //   println!("0b{:08b}", x );

 //   x = !(a | b);
 //   println!("X = !(a | b)");
 //   println!("0b{:08b}",x);
    
    
}