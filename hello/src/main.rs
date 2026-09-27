fn print_it(s: &String){
    println!("{}", s);
}

fn add_it(s: &mut String){
    s.push_str(" bye");
}

fn main() {
    let a = String::from("hello");
    let b = a;
    //println("{}", a);
    println!("{}", b);
    let c  = String::from("world");
    let d= c.clone();
    println!("{} {}", d, c);

    let e = 5;
    let f = e;
    println!("{} {}", e, f);
    let g = String::from("print");
    print_it(&g);
    print_it(&g);

    let mut h = String::from("Good");
    add_it(&mut h);
    println!("{}", h);

}
