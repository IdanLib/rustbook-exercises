fn is_only_alternating_signs(arr: &[i32]) -> bool {
    return true; 
}

fn main() {
    let arr1: [i32; 10] = [1, -2, 5, -14,6, -7,10, -1, 3, -10]; //true
    let arr2: [i32; 9] = [1, -2, 5, -14,6, -7,10, -1, 3]; //true
    let arr3: [i32; 10] = [-4, 1, -2, 5, -14,6, -7,10, -1, 3]; //true
    let arr4: [i32; 9] = [1, -2, 5, -14,6, 0,-10, 1, -3]; //false
    let arr5: [i32; 9] = [1, -2, 5, -14,6, -7,-10, 1, -3]; //false
    let arr6: [i32; 0] = []; //true
    let arr7: [i32; 2] = [1, -2]; //true
    let arr8: [i32; 9] = [1, -2, 5, -14,6, -7,10, -1, -3]; //false - odd

    
    println!("{}", &is_only_alternating_signs(&arr1));
    // println!(is_only_alternating_signs(arr2));
    // println!(is_only_alternating_signs(arr3));
    // println!(is_only_alternating_signs(arr4));
    // println!(is_only_alternating_signs(arr5));

}
