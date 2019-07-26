#[test]
#[allow(unused)]
fn test_char() {
    //8bit字符
    let c1 = '\x7f';
    println!("{}", c1.to_string());
    //unicode字符
    let c2 = '\u{7fff}';
    println!("{}", c2.to_string());
    //char类型占用4个字节,对于ASCII字符来说太多了,所以rust提供了单字节字符字面量,在字符或字符串前加b
    let c3: u8 = b'A';
    println!("{}", c3.to_string());
    let _: &[u8; 5] = b"Hello";
    //r#""#是raw string的语法
    let r: &[u8; 14] = br#"hello \n world"#;
    assert_eq!(14, r.len());
}

#[test]
#[allow(unused)]
fn test_integer() {
    //八进制字面量,这点和C,C++,js不一样
    let i1 = 0o11;
    //rust可以给任意类型添加方法,所以整数也是有方法的
    assert_eq!(729, 9_i32.pow(3))
}

#[allow(unused)]
fn arithmetic(m: i8, n: i8) {
    // 加法运算,有溢出风险
    println!("{}", m + n);
}

#[test]
#[allow(unused)]
fn test_overflow() {
    let m: i8 = 120;
    let n: i8 = 120;
    //默认情况下,debug版本编译的时候会进行整数溢出检查,release版本而是自动舍弃高位
//    arithmetic(m, n);
    //推荐使用下面的几个方法来显示的说明对溢出采取的行为
    let i = 100_i8;
    //check_*方法返回Option类型,如果溢出是None,其他是Some
    assert_eq!(None, i.checked_add(i));
    assert_eq!(Some(101), i.checked_add(1));
    //saturating_*在溢出的时候返回该类型范围中的最大值/最小值
    assert_eq!(127, i.saturating_add(i));
    //wrapping_*在溢出的时候直接抛弃溢出的最高位,将剩下的部分返回
    assert_eq!(-128, i.wrapping_add(28));
}

#[test]
#[allow(unused)]
fn test_float() {
    //几种浮点字面量
    let f1 = 123.0f64;
    let f2 = 0.1f64;
    let f3 = 0.1f32;
    let f4 = 12E+99_f64;
    let f5 = 2.;
    //浮点数的subnormal状态
    let mut small = std::f32::EPSILON;
    while small > 0.0 {
        small = small / 2.0;
        println!("{} {:?}", small, small.classify());
    }
    //Infinite和NaN
    let x = 1.0f32 / 0.0;
    let y = 0.0f32 / 0.0;
    println!("{} {}", x, y);//inf NaN
    //Infinite做算术运行的结果和想象的不一致
    let inf = std::f32::INFINITY;
    println!("{} {} {}", inf * 0.0, 1.0 / inf, inf / inf);//NaN 0 NaN
    //NaN有一个特殊性的性质,它不等于任何浮点数,包括它自己
    assert_ne!(std::f32::NAN, std::f32::NAN);
}

#[test]
#[allow(unused)]
fn test_type_convert() {
    let var1: i8 = 41;
    //rust使用as关键字进行类型转换
    let var2 = var1 as i16;
    //有时候需要多次转换
    let i = 42;
    let p = &i as *const i32 as *mut i32;
    println!("{:p}",p);
}