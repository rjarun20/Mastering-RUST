fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}", a);

    a.push(44);
    println!("{:?}", a);

    let idx:usize = 0;
    //why usize? because it is the type of index in vector and array in rust

    a[idx] = 321;
    println!("{:?}", a);

    //get
    // let el = a[10];
    // println!("{:?}", el);

    match a.get(3){
        Some(x) => println!("a[3] = {}", x),
        None => println!("No such element")
    }

    for x in &a{
        println!("{}", x);
    }

    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop();
    println!("Popped element is {:?}", last_elem);

    while let Some(x) = a.pop(){
        println!("{}", x);
    }

    println!("{:?}", a);


}

fn hash_map()
{
    use std::collections::HashMap;

    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("{:?}", shapes);

    //into
    let shape = shapes.get(&String::from("triangle"));
    println!("{:?}", shape);

    println!("{:?}", shapes["square".into()]);

    for (key, value) in &shapes{
        println!("{}: {}", key, value);
    }

    shapes.insert("square".to_string(), 5);
    println!("{:?}", shapes);

    shapes.entry("rectangle".into()).or_insert(4);

    shapes.entry("circle".to_string()).or_insert(1);
    {
        let actual = shapes
        .entry("circle".to_string())
        .or_insert(2);
        
        *actual = 0;
    }
    println!("{:?}", shapes);

    shapes.remove("circle");
    println!("{:?}", shapes);
}

fn hashset(){
    use std::collections::HashSet;

    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");

    println!("{:?}", greeks);

    greeks.insert("delta");
    println!("{:?}", greeks);

    let added = greeks.insert("delta");
    if added{
        println!("Added delta");
    }
    else{
        println!("Delta already exists");
    }

    if !greeks.contains("kappa"){
        println!("Kappa not found");
    }

    let _removed = greeks.remove("delta");
    if !greeks.contains("delta"){
        println!("Delta removed");
    }

    for greek in greeks{
        println!("{}", greek);
    }

    let _1_5:HashSet<i32> = (1..=5).collect();
    let _6_10:HashSet<i32> = (6..=10).collect();

    //subset
    let _1_10:HashSet<i32> = (1..=10).collect();
    println!("{:?}", _1_5.is_subset(&_1_10));
    println!("{:?}", _1_5.is_superset(&_1_10));



    let a:HashSet<i32> = [1,2,3].iter().cloned().collect();
    let b:HashSet<i32> = [4,5,6].iter().cloned().collect();

    let union:HashSet<&i32> = a.union(&b).collect();
    println!("{:?}", union);

    let difference:HashSet<&i32> = a.difference(&b).collect();
    println!("{:?}", difference);

    let intersection:HashSet<&i32> = a.intersection(&b).collect();
    println!("{:?}", intersection);

    let symmetric_difference:HashSet<&i32> = a.symmetric_difference(&b).collect();
    println!("{:?}", symmetric_difference);

    //disjoint
    println!("{:?}", a.is_disjoint(&b));
    //equal
    println!("{:?}", a == b);




}

fn iterators(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter{
        println!("Got: {}", val);
    }

    let v2:Vec<i32> = vec![1,2,3];
    let v2_iter = v2.iter();
    let total:i32 = v2_iter.sum();
    println!("Total: {}", total);

    let v3:Vec<i32> = vec![1,2,3];
    let v3_iter = v3.iter();
    let v3_iter = v3_iter.map(|x| x+1);
    let v3:Vec<i32> = v3_iter.collect();
    println!("{:?}", v3);

    let v4:Vec<i32> = vec![1,2,3];
    let v4_iter = v4.iter();
    let v4_iter = v4_iter.filter(|x| **x>1);
    let v4:Vec<&i32> = v4_iter.collect();
    println!("{:?}", v4);

    let v5:Vec<i32> = vec![1,2,3];
    let v5_iter = v5.iter();
    let v5_iter = v5_iter.filter(|x| **x>1);
    let v5_iter = v5_iter.map(|x| x+1);
    let v5:Vec<i32> = v5_iter.collect();
    println!("{:?}", v5);

    let v6:Vec<i32> = vec![1,2,3];
    let v6_iter = v6.iter();
    let v6_iter = v6_iter.filter(|x| **x>1);
    let v6_iter = v6_iter.map(|x| x+1);
    let v6_iter = v6_iter.collect::<Vec<i32>>();
    println!("{:?}", v6_iter);

    let mut v7:Vec<i32> = vec![1,2,3];
    let v7_iter = v7.iter_mut();

    for val in v7_iter{
        *val += 1;
    }

    println!("{:?}", v7);

    //extend
    let mut v8:Vec<i32> = vec![1,2,3];
    let v9:Vec<i32> = vec![4,5,6];
    v8.extend(v9);
    println!("{:?}", v8);





}

fn main() {
    println!("Hello, world!");
    vectors();
    hash_map();
    hashset();
    iterators();
}
