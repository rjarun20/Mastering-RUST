use std::fmt::Debug;

//implementing traits

trait Animal {
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Dog;
impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
    fn noise(&self) -> &'static str {
        "Bark"
    }
}

struct Cat;
impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat" //self.name
    }
    fn noise(&self) -> &'static str {
        "Meow" //self.
    }
}

trait summable<T> {
    fn sum(&self) -> T;
}

impl summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

impl summable<f64> for Vec<f64> {
    fn sum(&self) -> f64 {
        let mut result = 0.0;
        for x in self {
            result += *x;
        }
        result
    }
}

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Shape for Rectangle {
    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }
}

//fn print_area(s: impl Shape + Debug) 
// fn print_area<T: Shape + Debug>(s: T)
fn print_area<T>(shape:T) where T: Shape + Debug 
{
    println!("Area = {}", shape.area());
    println!("{:?}", shape);
}

#[derive(Debug)]
struct Person {
    name: String,
}
//into
impl Person {
    // fn new<S: Into<String>>(name: S) -> Person 
    fn new<T>(name: T) -> Person where T: Into<String>
    {
        Person { name: name.into() }
    }
}

//from
//drop
struct Creature{
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} goes away", self.name);
    }
}

//operator overloading
use std::ops::Add;

#[derive(Debug)]
struct Complex<T>
{
    re: T,
    im: T,
}

impl<T> Complex<T>
{
    fn new(re:T , im:T) -> Complex<T>{
        Complex::<T> {re, im}
    }
}

impl Add for Complex<i32>
{
    type Output = Complex<i32>;
    fn add (self , rhs:Self) -> Self::Output{
        Complex
        {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

//clone
//copy
//default
//iter
//iter_mut
//into_iter
//into_iter_mut
//from_iter
//from_iter_ref
//from_ref
//from_mut
//from_slice
//from_vec
//from_vec_mut
//from_vec_ref
//from_vec_ref_mut
//from_vec_mut_ref
use std::mem;
//static dispatch

trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("i32: {}", self);
    }
}

impl Printable for f64 {
    fn print(&self) {
        println!("f64: {}", self);
    }
}

fn print_it<T: Printable>(z: T) {
    z.print();
}

// //dynamic dispatch
fn print_it_dyn(z: &dyn Printable) {
    z.print();
}

//dynamic dispatch with array of different types
fn print_it_dyn_array(z: &[&dyn Printable]) {
    for i in z {
        i.print();
    }
}








fn main() {
    println!("Hello, world!");
    let dog = Dog;
    let cat = Cat;
    dog.talk();
    cat.talk();

    let v = vec![1, 2, 3, 4, 5];
    println!("Sum = {}", v.sum());

    let v = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    println!("Sum = {}", v.sum());

    let r = Rectangle { width: 10, height: 20 };
    print_area(r);

    let p = Person::new("John");
    println!("{:?}", p);
    
    let c = Creature::new("Orc");
    drop(c);
    println!("Game proceeds");

    let c1 = Complex::new(1, 2);
    let c2 = Complex::new(3, 4);
    let c3 = c1 + c2;
    println!("c3 = {:?} ", c3);

    let a = 123;
    let b = 1.23;
    print_it(a);
    print_it(b);

    let a = 123;
    let b = 1.23;
    print_it_dyn(&a);
    print_it_dyn(&b);





    




}
