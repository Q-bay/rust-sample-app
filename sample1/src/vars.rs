pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);
    let _i1 = 3;
    let _f1 = 1.0;

    println!("{}", usize::BITS);
    // pointerの表記は:p
    println!("Memory address of const is {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Memory address of i2 is {:p}", &i2);
    println!("Memory address of i3 is {:p}", &i3);
}
