use rand;

fn rng() -> i32 {
       let x: i32 = rand::random::<i32>();
       return x;
}

pub fn shuffle(size:i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < size{
        vec.push(rng());
        i += 1;
    }
    return vec
}

