use std::thread;
use std::time::Duration;

fn generate_workout(强度: u32, 随机数: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if 强度 < 25 {
        println!("Today, do {} pushups!", expensive_closure(强度));
        println!("Next, do {} situps!", expensive_closure(强度));
    } else {
        if 随机数 == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(强度));
        }
    }
}

fn main() {
    let 用户指定强度 = 10;
    let 用户指定随机数 = 7;

    generate_workout(用户指定强度, 用户指定随机数);

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;
    add_one_v3(5);
    add_one_v4(5);

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);     //error[E0308]: mismatched types

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // println!(": {:?}", list);

    let mut list = vec![1, 3, 5];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    #[derive(Debug)]
    struct  Rect {
        width: i32,
        height:  i32,
    }

    let mut list = vec![
        Rect {width: 8, height: 10},
        Rect {width: 10, height: 12},
        Rect {width: 3, height: 10},
    ];

    // list.sort_by_key(|r| r.width);
    let mut counts = 0;
    list.sort_by_cached_key(|r| {
        counts += 1;
        r.width
    });
    println!("{counts}, {:#?}", list)
}

