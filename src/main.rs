mod rng;

fn get_arguments(){
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", args);
}

fn main() { 
    let args = get_arguments();
    println!("{}", args.1.parse().unwrap());
    for i in rng::shuffle(size){
        println!("{}", i);
    }
}

