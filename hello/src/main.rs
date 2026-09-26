fn print_it(s: String) {
    println!("{}", s);
}

fn main() {
    let a = String::from("hello");
    print_it(a);
    println!("{}", a);
}
