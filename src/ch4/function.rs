fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn add2((x, y): (i32, i32)) -> i32 {
    x + y
}

#[test]
fn test_invoke() {
    let p = (1, 3);
    let func = add2;
    assert_eq!(add1(p), func(p));
}

#[test]
fn test_type() {
    let mut func = add1;
    //虽然它们的函数签名是一样的,但rust仍然认为是不同类型,所以下面的代码会报错
//    func = add2;
    //解决方法是让func转成通用的fn类型
    let mut func = add1 as fn((i32, i32)) -> i32;
    func = add2;
    //或者这样写:
    let mut func: fn((i32, i32)) -> i32 = add1;
    func = add2;
}

#[test]
fn test_inner() {
    //在函数内部可以定义静态变量,常量,函数,trait,类,模块,一般这些东西只限函数内部使用
    static INNER_STATIC: i64 = 32;
    fn inner_inc(x: i64) -> i64 {
        x + 1
    }
    struct InnerTemp(i64);
    impl InnerTemp {
        fn incr(&mut self) {
            self.0 = inner_inc(self.0)
        }
    }
    let mut t = InnerTemp(INNER_STATIC);
    t.incr();
    println!("{}", t.0);
}

#[test]
fn test_diverge() {
    //发散函数,特征就是返回值类型是!
    fn diverges() -> ! {
        panic!("This function never returns!")
    }
    //发散函数可以和任何类型兼容,类似Scala的Nothing
    let x: i32 = diverges();
    let y: String = diverges();
    //这种类型存在的原因是为了一些有分支表达式的分支类型的一致
}

//const fn在编译期被执行,返回的值也被认为是常量
#[feature(const_fn)]
const fn cube(num: usize) -> usize {
    num * num * num
}

#[test]
fn test_const_fn() {
    const DIM: usize = cube(2);
    const ARR: [i32; DIM] = [0; DIM];
    println!("{:?}", ARR);
}

#[test]
fn test_recursive() {
    fn fib(index: u32) -> u32 {
        if index == 1 || index == 2 {
            1
        } else {
            fib(index - 1) + fib(index - 2)
        }
    }

    println!("{}", fib(8))
}
