mod rng;

fn get_arguments() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    return args;
}

fn main() { 
    let args = get_arguments();
    let size = &args[1];
    let mut unsorted_list = rng::shuffle(size.parse::<i32>().unwrap());
    for i in &unsorted_list {
        println!("{}", i);
    }

    println!("\n");

    unsorted_list.sort();
    for i in &unsorted_list {
        println!("{}", i);
    }

}

