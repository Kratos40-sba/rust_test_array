use rand::prelude::*;
fn gen_rand_arr<const SIZE: usize>(rng: &mut ThreadRng, n: u64) -> [u64; SIZE] {
    let mut arr = [0; SIZE];
    for x in &mut arr {
        *x = rng.gen_range(1..n);
    }
    arr
}
#[macro_use]
extern crate timeit;

fn main() {
    let mut rng = thread_rng();
    let massive_array :[u64;100000] =gen_rand_arr(&mut rng,10);
    let mut i = 0;
    let mut sum : u64 = 0 ;
    let sec = timeit_loops!(10000, {
   while i < massive_array.len(){
            sum = sum + massive_array[i];
            i = i+1;
    }
    });
    println!("{} nanosecond",sec*1e9)


}

//timeit!({
// while i < massive_array.len(){
//  sum = sum + massive_array[i];
// i = i+1;
// }
//});