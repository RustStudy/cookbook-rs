use ::rand::{Rng, Rand};
use ::rand::distributions::{Normal, IndependentSample};


// 生成随机浮点数
pub fn gen_f64_rand() -> f64{
    let mut rng = ::rand::thread_rng();
    println!("Random f64: {}", rng.gen::<f64>());
    rng.gen::<f64>()
}

// 从范围中产生随机数
pub fn gen_numbder_from_range() -> usize {
    let mut rng = ::rand::thread_rng();
    println!("{}", rng.gen_range(0, 10));
    rng.gen_range(0, 10)
}

// 正态分布来生成随机数
pub fn gen_with_distribution() -> f64 {
    let normal = Normal::new(3.0, 5.0);
    let mut rng = ::rand::thread_rng();
    let v = normal.ind_sample(&mut rng);
    println!("{} is from a N(3, 25) distribution", v);
    v
}

// 为自定义类型生成随机值
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Rand for Point {
    fn rand<R: Rng>(rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point { x: rand_x, y: rand_y }
    }
}
