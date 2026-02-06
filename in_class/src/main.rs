fn get_rgb(c: char) -> (u8, u8, u8) {
   match c {
         'R' => (255, 0, 0),
         'G' => (0, 255, 0),
         'B' => (0, 0, 255),
         _ => (0, 0, 0), // default case for unsupported characters
   }
}

fn main() {
    // we are going to accept a letter like RGB
    // and we should return 
    // RED tuple (255,0,0)
    // GREEN tuple (0,255,0)
    // BLUE tuple (0,0,255)

    // write a function which accepts char 'R' G B
    // ans return above specified tuple

    let red = color_tuple('R');
    let green = color_tuple('G');
    let blue = color_tuple('B');

    let letters = ['R', 'G', 'B',];

    for idx in 0..letters.len(){
        let color = get_rgb(letters[idx]);
        println!("{:?}", 1, color);
    }


    println!("Red: {:?}", red);
    println!("Green: {:?}", green);
    println!("Blue: {:?}", blue);
    
}