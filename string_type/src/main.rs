fn strtype(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    //utf8
    let s:&'static str = "Hello, world!";
    for c in s.chars(){
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0){
        println!("first letter is {}", first_char);
    }

    //bytes
    for b in s.bytes(){
        println!("{}", b);
    }

    //concatenation
    let mut letters = String::new();
    let letter = "a";
    letters.push_str(letter);
    letters.push('b');
    println!("{}", letters);

    //heap string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    //&str to String
    let s1 = "foo".to_string();
    let s2 = "bar".to_string();
    println!("{}", s1 + &s2);

    //format
    let name = "Peter";
    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting);

    //format with {0}{1}
    let name = "Peter";
    let age = 27;
    let greeting = format!("Hello, {0}! You are {1} years old.", name, age);
    println!("{}", greeting);




    //split
    let s = "hello world welcome to rust programming";
    let mut iter = s.split_whitespace();
    assert_eq!(Some("hello"), iter.next());
    assert_eq!(Some("world"), iter.next());
    assert_eq!(Some("welcome"), iter.next());
    assert_eq!(Some("to"), iter.next());
    assert_eq!(Some("rust"), iter.next());
    assert_eq!(Some("programming"), iter.next());
    assert_eq!(None, iter.next());
    
    //lines
    let s = "hello\nworld";
    let mut iter = s.lines();
    assert_eq!(Some("hello"), iter.next());
    assert_eq!(Some("world"), iter.next());
    assert_eq!(None, iter.next());
    
    //trim
    let s = " hello world\n";
    assert_eq!("hello world", s.trim());
    assert_eq!("hello world\n", s);
    
    //replace
    let s = "hello world";
    assert_eq!("hello rust", s.replace("world", "rust"));
    assert_eq!("hello world", s);
    
    //contains
    let s = "hello world";
    assert!(s.contains("world"));
    
    //starts_with
    let s = "hello world";
    assert!(s.starts_with("hello"));

    //ends_with
    let s = "hello world";
    assert!(s.ends_with("world"));

    //is_empty
    let s = "";
    assert!(s.is_empty());

    //len
    let s = "hello world";
    assert_eq!(11, s.len());

    //




}

fn number_guessing_game(){
    use rand::Rng;
    use std::io::stdin;

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("Please input your guess.");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
    // strtype();
    number_guessing_game();
}
