#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    
   let calc_area = Rectangle {
        width: 12,
        height: 20,
    };

    println!("The area of this rectangle is {}", area(&calc_area));
    // println!( "calc_area is {:#?}", calc_area );
    dbg!(&calc_area);



    fn area (calc_area: &Rectangle) -> u32{

        calc_area.width * calc_area.height
    }
    
 }


