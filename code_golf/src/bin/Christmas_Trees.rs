
fn main(){
    
    for trees in 3..10{
        for dot in 0..trees{
            print!(" {: <1$}","", trees-dot-1);
            println!("{:*<1$}","",1+2*dot);
        }
        println!(" {: <1$}*\n", "",trees-1)
        
    }

}