pub mod ex1 {

use std::collections::HashMap;

fn find_median(list: &mut Vec<i32>) -> Option<i32>{
    list.sort();
    
    //option 1
    // let median_opt:Option5<&i32> = list.get(list.len()/2);
   
    // return match median_opt  {
    //     Some(median) => *median,
    //     None => panic!("No median found!"),
    // }

    //option 2
    return list.get(list.len()/2).copied();
}

fn find_mode(list: &Vec<i32>) -> Option<i32> {
    let mode: Option<&i32> = list.get(0);
    if let None = mode {
        return mode.cloned();
    }

    let mut count_map = HashMap::<i32, i32>::new();
    let mut max_count = 0; 

    let mut mode_as_i32 = mode.unwrap().clone(); 
    
    //1. one loop
    for item in list {
        let count = count_map.entry(*item);
        let key = *count.key(); 
        let val = *(count.or_insert(0)) + 1;

        if max_count < val {
            mode_as_i32 = key;
            max_count = val;
        }
    }

    return Some(&mode_as_i32).cloned();
    //2. two loops
    // for item in list {
    //     let count = count_map.entry(*item).or_insert(0);
    //     *count += 1;
    // }

    // for (key, val) in count_map.iter() {
    //     if *val > max_count {
    //         mode = *key;
    //         max_count = *val;
    //     }
    // }
}

pub fn solve_ex1() -> () {
    println!(">> Chapter 8, exercise 1: Find median and mode of given list. <<");
    let mut list1: Vec<i32> = vec![3,6,3, -7, -13, 0, 4, 12, -6, -13, 1, 23, -2, 14, 61, -13,-13,12,-13];
    let mut list2: Vec<i32> = vec![];

    println!("1. With full list:");
    println!("List is {:?}", list1);
    println!("Median is {:?}", find_median(&mut list1)); //.unwrap() to get inner value, but will panic if result is None.
    println!("Mode is {:?}", find_mode(&mut list1));

    println!("2. With Empty list, with error handling: ");
    println!("List is {:?}", list2);
    println!("Median is {:?}", find_median(&mut list2));
    println!("Mode is {:?}", find_mode(&mut list2));
    }
}