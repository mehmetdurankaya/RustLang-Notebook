

enum Color{
    Red,
    Blue,
    Green,
    RGBColor(u8,u8,u8)
}

pub fn enums(){
    let c=Color::RGBColor(0,0,0);
    match c {
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::Red => println!("r"),
        Color::RGBColor(0,0,0)=>println!("black"),
        _=>println!("the other color")
    }

}

