fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first: &i32 = &v[0];
    println!("The first element is {first}");
    v.push(6);

    

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i *= 5;
    }
    
    println!("{:?}", v);
}
