fn draw_tree(height: u8) {
    for i in 0..height {
        print!("{}", " ".repeat((height - i - 1) as usize));

        for j in 1..=i + 1 {
            print!("{} ", j);
        }

        println!();
    }
    println!();
}

fn main() {
    draw_tree(5);
}
