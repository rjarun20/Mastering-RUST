fn product(x: i32, y: i32) -> i32 {
    x * y// no semicolon here because we want to return the value of the expression
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}



fn functions(){
    let mut x = 5;
    println!("The value of x is: {}", x);

    increase(&mut x);

    println!("The value of x is: {}", x);
    
    let x = 5;
    let y = 6;
    println!("The product of x and y is: {}", product(x, y));
    println!("The sum of x and y is: {}", sum(x, y));

    //create a point and line struct  and implement methods for them
    #[derive(Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 5.0 };

    println!("{}", p1.distance(&p2));


    struct Line {
        start: Point,
        end: Point,
    }

    impl Line {
        fn length(&self) -> f64 {
            self.start.distance(&self.end)
        }
    }

    let l = Line { start: p1.clone(), end: p2.clone() };
    println!("{}", l.length());

    //implementing a method that takes ownership of the struct
    impl Line {
        fn to_string(self) -> String {
            format!("Line from ({}, {}) to ({}, {})", self.start.x, self.start.y, self.end.x, self.end.y)
        }
    }

    let l = Line { start: p1.clone(), end: p2.clone() };
    println!("{}", l.to_string());

    //implementing a method that takes a mutable reference to the struct
    impl Line {
        fn set_start(&mut self, x: f64, y: f64) {
            self.start = Point { x: x, y: y };
        }
    }

    let mut l = Line { start: p1, end: p2 };
    l.set_start(1.0, 2.0);
    println!("{}", l.to_string());




}

fn increase(x: &mut i32) {
    *x += 1;
}

fn say_hello(){
    println!("Hello, world!");
}

//closures
fn closure(){

    let hello = say_hello;
    hello();

    let plus_one = |x: i32| -> i32 { x + 1};
    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };
    assert_eq!(4, plus_two(2));

    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(10, num);

    let two = 2;
    let plus_two = |x| {
        let mut result: i32 = x;
        result += two;
        result
    };
    assert_eq!(4, plus_two(2));

    //scope and closure
    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(5, num);

    //closure that takes a closure as an argument
    fn call_with_one<F>(some_closure: F) -> i32
        where F: Fn(i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);

    //closure that returns a closure
    fn factory() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);

    //closure that captures variables from the environment
    let num = 5;
    let plus_num = |x: i32| x + num;
    assert_eq!(10, plus_num(5));
}

//higher order functions
fn higher_order_functions(){
    fn is_even(x: i32) -> bool {
        x % 2 == 0
    }

    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let square = i * i;
        if square > limit {
            break;
        } else if is_even(square) {
            sum += square;
        }
    }
    println!("{}", sum);

    let limit = 500;
    let sum = (0..)
        .map(|x| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("{}", sum);

    //fn signature
    fn foo<F: Fn(i32) -> i32>(closure: F) -> i32 {
        closure(1)
    }

    let bar = |x| x + 2;
    assert_eq!(foo(bar), 3);

    //fn signature with trait object
    fn foo3(closure: &dyn Fn(i32) -> i32) -> i32 {
        closure(1)
    }

    let bar = |x| x + 2;
    assert_eq!(foo(&bar), 3);

    //fn signature with trait object and type alias
    type Closure = dyn Fn(i32) -> i32;
    fn foo1(closure: &Closure) -> i32 {
        closure(1)
    }

    let bar = |x| x + 2;
    assert_eq!(foo(&bar), 3);

    //fn signature with trait object and type alias and where clause
    // type Closure = dyn Fn(i32) -> i32;
    // fn foo2(closure: &Closure) -> i32 {
    //     closure(1)
    // }

    let bar = |x| x + 2;
    assert_eq!(foo(&bar), 3);

    



}


fn main() {
    println!("Hello, world!");
    functions();
    closure();
}
