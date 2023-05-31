/* // 1: random number
// 2: user input
// 3: guess number , product number
// 4: input string is not int type.



fn main() {
    use rand::Rng;
    let random_number = rand::thread_rng().gen_range(0..=100);
    
    loop {
        use std::io::stdin;
        let mut s = String::new();
        stdin().read_line(&mut s).expect("failed read line");

        let s: i32 = match s.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
    
        use std::cmp::Ordering;
        match s.cmp(&random_number) {
            Ordering::Equal => {
                println!("Equal");
                break;
            },
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less"),
            
        }
    }
}
 */
fn main() {
    // let s1 = String::from("hello world");
    
    // let word = first_word(&s1);
    // println!("{}", word);


    let a = vec![10, 20, 30, 40, 50];
    let mut b:Vec<i32> = vec![];
    let mut n = 0;
    let length = a.len();
    // println!("{length}");
    loop {
        // b[n] = a[n];
        println!("{n}");
        // b[n] = a[length-n-1];
        b.push(a[length-1-n]);  
        n +=1;
        if n > length - 1 {
            break;
        }
    }
    println!("{:?}", b);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    (s, length)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

