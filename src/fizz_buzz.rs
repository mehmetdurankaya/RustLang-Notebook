pub fn fizz_buzz(){
    for sayi in 1..=100 {
        if(sayi % 3 == 0 && sayi % 5 == 0){
            println!("fizzBuzz");
        }else if (sayi % 3 == 0){
            println!("fizz");
        }else if(sayi % 5 == 0){
            println!("buzz");
        }
    }
}