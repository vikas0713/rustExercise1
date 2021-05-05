

fn swap_number(x:i32, y:i32) {
    let mut x = x;
    let mut y = y;
    println!("Current values x: {} , y: {}", x, y);
    let mut temp = 0;
    temp = x;
    x = y;
    y = temp;
    println!("Swapped values x: {}, y: {}", x, y);
}
