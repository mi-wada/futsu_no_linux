fn main() {
    let args = {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);
        args
    };

    for arg in args.iter() {
        println!("{}", arg);
    }
}
