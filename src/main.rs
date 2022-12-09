mod rng;
mod sort;

fn get_arguments(){
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", args);
}

fn main() { 
    let args = get_arguments();
    let size = args.1.parse().unwrap();
    let unsorted_list = rng::shuffle(size);
    for i in unsorted_list {
        println!("{}", i);
    }

    let sorted_list = sort::sort(unsorted_list);
    for i in sorted_list {
        println!("{}", i);
    }

}

