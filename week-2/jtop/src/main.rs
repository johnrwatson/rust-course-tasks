// cargo run -- target 5

fn get_arguments<'a>() -> String {
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    println!("{:?}", args);
    let first_arg = args.clone().into_iter().nth(0);
    if let Some(arg) = first_arg {
        return arg
    }
    return "".to_string()
}

fn main() {
    let first = get_arguments();
    println!("{}", &first);
}