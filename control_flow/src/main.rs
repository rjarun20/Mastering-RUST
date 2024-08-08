#![allow(unused_must_use)]
#[allow(dead_code)]
#[allow(unused_imports)]


// use rand::Rng;
use std::io::stdin;
use std::string;


fn if_statement()
{
    let temp : i8 = 35;

    if temp > 30
    {
        println!("Really Hot outside!");
    }
    else if temp < 10
    {
        println!("Really Cold outside!");
    }
    else
    {
        println!("Temperature is normal outside.");
    }

    // expression
    let day = if temp > 20 {"sunny"} else if temp < 10 {"cloudy"} else {"partly cloudy"};
    println!("Today is {}", day);
}

fn while_loop()
{
    let mut x = 1;
    while x < 20
    {
        x *= 2;
        if x == 64
        {
            continue; // skip the rest of the loop body and start a new iteration of the loop 
        }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10
        {
            break; // exit the loop 
        }
    }
}


fn for_loop(){
    for x in 1..11
    {
        if x == 3
        {
            continue;
        }
        if x == 8
        {
            break;
        }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{}: {}", pos, y);
    }
}


//Match statement
fn match_statement()
{
    let country_code = 44;
    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown", // range
        _ => "Invalid"
    };
    println!("The country with code {} is {}", country_code, country);
}

// Option

fn option()
{
    let x = 3.0;
    let y = 2.0;

    // Some(z) -> Some value
    // None -> No value
    let result: Option<f64> = if y != 0.0 {Some(x/y)} else {None};

    println!("{}/{} = {}", x, y, result.unwrap());

    match result
    {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot divide {} by {}", x, y)
    }

    // if let Some(z) = result {println!("z = {}", z);}
}

#[derive(Debug)]
enum State {
    Locked,
    Failed,
    Unlocked
}


// //Combinaiton lock
fn combination_lock()
{
    let code = String::from("1234");
    let mut state: State = State::Locked;
    let mut entry = String::new();

    loop{
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input){
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry){
                    state = State::Failed;
                    continue;
                }

            }

            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;
            }

            State::Unlocked => {
                println!("Unlocked");
                return;
            }
        }
    }
    
 
}


fn main() {
    println!("Control Statment in Rust");

    //if_statement();
    //while_loop()
    //for_loop();
    //match_statement();
    // option();
    combination_lock();

}
