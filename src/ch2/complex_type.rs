#[test]
#[allow(unused)]
fn test_tuple() {
    //元组,语法和scala类似
    let a = (1, false);
    let b = ("a", (1, 2));
    //如果只有一个元素,需要在这个元素后面加一个逗号来和括号表达式区分
    let c = (0, );
    //访问元组中元素的方法
    //模式匹配
    let p = (1, 2);
    let (a, b) = p;
    //数字索引
    let x = p.0;
    let y = p.1;
    //空元组叫做unit,和scala中类似,它所占内存是0,和空构造体一样
    use std::mem::size_of;
    struct Person;
    assert_eq!(1, size_of::<i8>());
    assert_eq!(4, size_of::<char>());
    assert_eq!(0, size_of::<()>());
    assert_eq!(0, size_of::<Person>());
}

#[test]
#[allow(unused)]
fn test_struct() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };
    println!("Point is at {} {}", p.x, p.y);
    //使用模式匹配访问struct内部元素
    let Point { x, y } = p;
    println!("Point is at {} {}", x, y);
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    fn default() -> Point3D {
        Point3D { x: 0, y: 0, z: 0 }
    }
    //struct可以使用另一个struct的部分成员
    let origin = Point3D { x: 5, ..default() };
    assert_eq!(0, origin.y);
    let point = Point3D { z: 1, x: 2, ..origin };
    assert_eq!(0, point.y);
}

#[test]
#[allow(unused)]
fn test_tuple_struct() {
    struct Color(i32, i32, i32);
    //tuple struct的属性名称实际是index
    let c = Color { 0: 1, 1: 2, 2: 3 };
}

#[test]
#[allow(unused)]
fn test_enum() {
    //枚举的定义,其中的成员可以像空构造体不指定类型,也可以像tuple struct,也可以像一般的struct
    enum Number {
        Int(i32),
        Float(f32),
    }
    fn read_num(num: &Number) {
        match num {
            &Number::Int(value) => println!("Integer {}", value),
            &Number::Float(value) => println!("Float {}", value),
        }
    }
    let n = Number::Int(1);
    read_num(&n);
    //enum的包含额外的tag信息来说明当前变量属于什么类型
    use std::mem::size_of;
    assert_eq!(8, size_of::<Number>());//按理说应该是4个byte,但因为有tag信息,所以多了4个byte
    enum Animal {
        Dog = 1,
        Cat = 200,
        Tiger,
    }
    //可以自己指定变体的标记值
    let x = Animal::Tiger as isize;
    println!("Tiger {}", x);

    //Option是标准库中最常用的enum

    //enum中的变体实际只是一个名字,但可以把他们当成类型构造器使用,所以也可以把它们当成一个函数来使用
    let arr = [1, 2, 3, 4, 5];
    let v: Vec<Option<&i32>> = arr.iter().map(Some).collect();
    println!("{:?}", v);
}

#[test]
#[allow(unused)]
fn test_type_recur() {
    //rust支持类型递归,但是需要把产生递归的那个类型改成指针
    struct Recursive {
        data: i32,
        r: Box<Recursive>,
    }
}
