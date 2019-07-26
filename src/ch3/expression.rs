#[test]
#[allow(unused)]
fn test_assignment() {
    //rust中的赋值表达式的返回值是(),和scala一样
    let mut x = 1;
    assert_eq!((), x = 2);
    //rust支持运算符和赋值表达式组合
    let y = 3;
    x += y;
    x *= y;
    assert_eq!(15, x);
    //rust不支持++和--
}

#[test]
#[allow(unused)]
fn test_arithmetic_exp() {
    let num1 = 0b_1010_1010;
    let num2 = 0b_1111_0000;
    println!("{:08b}", !num1);
    println!("{:08b}", num1 & num2);
    println!("{:08b}", num1 | num2);
    println!("{:08b}", num1 ^ num2);
    println!("{:08b}", num1 << 4);
    println!("{:08b}", num1 >> 4);
}

#[test]
#[allow(unused)]
fn test_if_else() {
    //if-else可以作为表达式,同scala
    let s = if 1 == 1 { 1 } else { 2 };
}

#[test]
#[allow(unused)]
fn test_loop() {
    let mut count = 0u32;
    println!("Let's count util infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("Ok,that's enough");
            break;
        }
    }
    //break和continue支持跳转到指定层级
    let mut m = 1;
    let n = 1;
    'a: loop {
        if m < 100 {
            m += 1;
        } else {
            'b: loop {
                if m + n > 50 {
                    println!("break!");
                    break 'a;
                } else {
                    continue 'a;
                }
            }
        }
    }
    //loop也可以作为表达式
    let v = loop {};
}

#[test]
#[allow(unused)]
fn test_while() {
    let mut x = 0;
    while x < 10 {
        if x == 5 {
            break;
        }
        x += 1;
    }
    println!("{}", x);
}

#[test]
#[allow(unused)]
fn test_for() {
    let array=&[1,2,3,4];
    for i in array {
        println!("{}",i)
    }
}
