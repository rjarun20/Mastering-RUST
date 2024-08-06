#[allow(dead_code)]

#[derive(Debug)]
struct Point{
    x:f64,
    y: f64
}


struct Line{
    start: Point,
    end: Point
}

fn cordinates()
{
    let p1 = Point{x:3.0 , y:4.0};
    let p2 = Point{x:5.0 , y:10.0};
    let line = Line{start:p1, end:p2};

    println!("Start : {:?} end: {:?}", line.start, line.end);
}

//enums
#[derive(Debug)]
enum Colour{
    Red,
    Green,
    Blue,
    RGB(u8,u8,u8),
    //create struct
    CMYK {cyan:u8, magenta:u8, yellow:u8, black:u8}
}

fn colour (){
    // let c:Colour = Colour::RGB(0,0,0);
    //test for (r,g,b) = 10,20,30
    let c:Colour = Colour::RGB(10,20,30);
    println!("Colour is {:?}", c);

    match c{
        Colour::Red => println!("Red"),
        Colour::Green => println!("Green"),
        Colour::Blue => println!("Blue"),
        Colour::RGB(0,0,0) => println!("Black"),
        Colour::RGB(r,g,b) => println!("Red: {}, Green: {}, Blue: {}", r,g,b),
        Colour::CMYK{cyan:_, magenta:_, yellow:_, black:255} => println!("Black"),
        _ => println!("Some other colour")

    }
}


//uniion
union IntOrFloat{
    i: i32,
    f: f32
}

fn main() {
    println!("Hello, world!");
    cordinates();
    colour();
    let mut iof = IntOrFloat{i:123};
    unsafe{
        println!("{}", iof.i);
    }
    iof.f = 234.33;
    unsafe{
        println!("{}", iof.f);
    }

    let x = 6;
    let y = 0;

    let result = if y!=0 {Some(x/y)} else {None};

    match result{
        Some(val) => println!("Result is {}", val),
        None => println!("Cannot divide by zero")
    }

    if let Some(val) = result{
        println!("Result is {}", val);
    }
    else{
        println!("Cannot divide by zero");
    }

    let mut a:[i32;5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a has {} elements, first is {}", a.len(), a[0]);

    for i in 0..a.len(){
        println!("{}", a[i]);
    }

    for e in a.iter(){
        println!("{}", e);
    }

    for e in &a{
        println!("{}", e);
    }

    let sl = &a[1..4];
    for e in sl.iter(){
        println!("{}", e);
    }

    let sl = &a[1..];
    for e in sl.iter(){
        println!("{}", e);
    }

    //matrix
    let mtx:[[f32;3];2] = [
        [1.0,2.0,3.0],
        [4.0,5.0,6.0]
    ];

    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i==j{
                println!("mtx[{}][{}] = {}", i,j, mtx[i][j]);
            }
        }
    }

    //slices of an array
    let arr = [1,2,3,4,5];
    let sli = &arr[1..4];
    for e in sli.iter(){
        println!("{}", e);
    }
    //mutable slice
    let mut arr = [1,2,3,4,5];
    let sli = &mut arr[1..4];
    sli[0] = 6;
    for e in sli.iter(){
        println!("{}", e);
    }



    //tuples
    let tpl = (1,2,3);
    let tpl2 = (4,5,6);
    let tpl3 = (1,2,3,4,5,6,7,8,9,10);
    let (x,y,z) = tpl;
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
    println!("3rd element of tpl3 is {}", tpl3.2);
    //accessing tuple of tuples
    let tpl4 = (tpl, tpl2);
    println!("2nd element of tpl4 is {:?}", tpl4.1);
    println!("2nd element of 2nd element of tpl4 is {}", (tpl4.1).1);



    //pattern matching
    let age:u8 = 20;
    if age<21{
        println!("Cannot drink");
    }
    else{
        println!("Can drink");
    }

    match age{
        0 => println!("Not born yet"),
        1..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        _ => println!("Adult")
    }

    //match with OR and AND , with "|" and ","
    let name = "John";
    match name{
        "John" | "Jane" => println!("Name is John or Jane"),
        "John" if age<21 => println!("John cannot drink"),
        _ => println!("Some other name")
    }

    

    let can_drink = true;
    let age = 18;
    match (age, can_drink){
        (0..=20, false) => println!("Cannot drink"),
        (0..=20, true) => println!("Can drink"),
        _ => println!("Can drink")
    }

    //point (0,0) , (3,4)
    match (3,4){
        (0,0) => println!("Origin"),
        (x,0) => println!("x-axis"),
        (0,y) => println!("y-axis"),
        (x,y) => println!("({}, {})", x,y),
        (_,y) => println!("(0, {})", y),
        (x,_) => println!("({}, 0)", x),
        (_,_) => println!("Some other point"),
    }


    //Generics
    fn get_larger<T:PartialOrd>(a:T, b:T)->T{
        if a>b{
            a
        }
        else{
            b
        }
    }

    println!("Larger of 1 and 2 is {}", get_larger(1,2));
    println!("Larger of 1.1 and 2.2 is {}", get_larger(1.1,2.2));
    println!("Larger of 'a' and 'b' is {}", get_larger('a','b'));

    struct Point<T>{
        x:T,
        y:T
    }

    let a:Point<i32> = Point{x:0, y:0};
    let b:Point<f32> = Point{x:0.0, y:0.0};


    
    struct Point<T,V>{
        x:T,
        y:V
    }

    let a:Point<i32,f32> = Point{x:0, y:0.0};
    let b:Point<f32,i32> = Point{x:0.0, y:0};

    


    //structs
    struct Point{
        x:f64,
        y:f64
    }

    struct Line{
        start: Point,
        end: Point
    }

    let p1 = Point{x:3.0, y:4.0};
    let p2 = Point{x:5.0, y:10.0};
    let line = Line{start:p1, end:p2};

    println!("Start: ({}, {}), End: ({}, {})", line.start.x, line.start.y, line.end.x, line.end.y);

    







}
