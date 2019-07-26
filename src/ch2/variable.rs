extern crate lazy_static;
use lazy_static::*;
use std::collections::HashMap;
#[test]
#[allow(unused)]
fn test_variable_declare() {
    //mut x实际上是一个模式,用于解构
    let mut x = 1;
    x = 10;
    //另一种模式
    let (mut a, mut b) = (1, 2);
    assert_eq!(1, a);
    assert_eq!(2, b);

    //raw identifier,当需要使用rust保留字作为变量名的时候使用
    let r#if = 1;
    assert_eq!(1, r#if);

    //rust中下划线的用法和scala中的类似,表示忽略这个变量绑定
    let (_, c) = (3, 4);
    assert_eq!(4, c);

    //rust的强大的类型推导
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
    //可以在声明的时候写部分类型,剩下的让编译器推导
    let player_scores = [
        ("Jack", 20), ("Jane", 23),
        ("Jill", 18), ("John", 19)
    ];
    let players: Vec<_> = player_scores
        .iter()
        .map(|&(player, _score)| {
            player
        }).collect();
    println!("{:?}", players);
}

//类型别名
#[allow(unused)]
type Age = u32;
//带泛型的类型别名
#[allow(unused)]
type Double<T>=(T,Vec<T>);

#[allow(unused)]
fn grow(age: Age, year: u32) -> Age {
    age + year
}

#[test]
fn test_type_alias() {
    let x:Age=20;
    println!("20 years later: {}",grow(x,20));
}

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        println!("Init HASHMAP...");
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

#[test]
fn test_static_var() {
    //静态变量初始化的时候必须使用编译期能确定的值,并且必须声明的时候就初始化
    static G1: i32 = 0;
    println!("{}", G1);
    //可以使用mut来修饰静态变量,但读写的时候需要放在unsafe代码块中
    static mut G2: i32 = 3;
    unsafe {
        G2 = 5;
        println!("{}", G2);
    }
    //静态变量初始化只能使用const函数,不能使用普通函数,这和Java是一样的
    use std::sync::atomic::AtomicBool;
    static FLAG: AtomicBool = AtomicBool::new(true);
    println!("{:?}", FLAG);

    //使用lazy-static库来声明需要较长初始化时候的静态变量
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
}

#[test]
#[allow(unused)]
fn test_const_var() {
    //常量不能用mut修饰,并且以const声明一个变量,也不具备类似let的模式匹配功能
    const GLOBAL: (i32, i32) = (1, 2);
}