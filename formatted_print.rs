fn main() {
    println!("{} day", 1);
    println!("{0} or {1}: {0}", "Yes", "No");
    println!("{blue} {green} {red}",
             red = "red",
             green = "green",
             blue = "blue");
    println!("binary representation: {:b}", 7);
    println!("right align fill with spaces:{:>5}", 3);
    println!("right align fill with zeros:{:0>5}", 3);
    println!("right align fill with zeros:{:0>1$}", 3, 5);
    println!("right align fill with zeros:{:0>width$}", 3, width = 5);
}