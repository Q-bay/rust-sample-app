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

    let y = 2;
    println!("Memory address of y is {:p}", &y);
    let y = y + 1;
    println!("Memory address of y is {:p}", &y);
    let y = y * 2;
    println!("Memory address of y is {:p}", &y);
    println!("value of y is {}", y);
    {
        let y = 0;
        println!("value of y is {}", y);
    }
    println!("value of y is {}", y);

    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("value of t1 is {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    println!("{:?}", t2);
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    // タプルの出力には?が必要
    println!("{:?}", t2);
}
