pub mod ex3 {
    use std::collections::HashMap;
    
    #[derive(Debug)]
    enum OperationStatus<'a> {
        Success(&'a str),
        Failure(&'a str)
    }
    
fn add_employee<'a>(instruction: &'a str, personnel: &'a mut HashMap<&'a str, Vec<&'a str>>) -> OperationStatus<'a> {

    if !instruction.starts_with("Add") {
        return OperationStatus::Failure("Invalid Command: Use 'Add' as the first word.");
    }

    let instr_as_vec: Vec<&str> = instruction.split(' ').collect();

    if instr_as_vec.len() != 4 {
        return OperationStatus::Failure("Invalid Command: Use pattern 'Add {name} to {department}'");
    }
    
    let name = *instr_as_vec.get(1).unwrap();
    let department = *instr_as_vec.get(3).unwrap();

    match personnel.get_mut(department) {
        Some(dept_ppl) => {
            dept_ppl.push(name);
            dept_ppl.sort();
            println!("{:?}", dept_ppl);
        },
        None => {
            personnel.insert(department, vec![name]);
        },
    }

    // if let Some(dept_ppl) = personnel.get(department) {
    //     dept_ppl.push(name);
    //     dept_ppl.sort();
    // }

    return OperationStatus::Success("Added Successfully");
}


fn get_personnel_by_dept(dept: &str, personnel: &HashMap<&str, Vec<&str>>) -> String {
    
    // let mut employees_in_dept: String = String::new();
    let mut employees_in_dept = format!("{dept}: ");

    // //1. With if let 
    // if let Some(current_dept)= personnel.get(dept) {
    //     for emp in current_dept {
    //         employees_in_dept.push_str(emp);
    //         employees_in_dept.push(' ');
    //     }
    //     return employees_in_dept;
    // } 
    
    // return String::from("Invalid department name. Please check again.");


    //2. With match
    match personnel.get(dept) {
        Some(current_dept) => {

            for emp in current_dept {
                employees_in_dept.push_str(emp);
                employees_in_dept.push_str(", ");
            }
            return employees_in_dept;
        },
        None => return String::from("Invalid department name. Please check again.")
    }

}

fn get_all_personnel(personnel: &HashMap<&str, Vec<&str>>) -> String {
    
    let mut all_personnel = String::from("\nFull Personnel: \n---------------\n"); 

    for dept in personnel.keys() {
        let mut temp = get_personnel_by_dept(*dept, &personnel);
        temp.push('\n');
        all_personnel.push_str(&temp);
    }

    return all_personnel;
}

pub fn solve_ex3() -> () {
    
    println!("\n>> Chapter 8, exercise 3: <<");
    println!(">> Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.<< ");

    let mut personnel1: HashMap<&str, Vec<&str>> = HashMap::new();

    personnel1.insert("Engineering", vec!["Lisa", "Ziltoid", "Marge", "Homer"]);
    personnel1.insert("Music", vec!["Lars", "James", "Robert", "Kirk"]);
    personnel1.insert("Marketing", vec!["Yoshi", "Mario", "Luigi", "Krupa"]);

    
    //must clone due to borrow rules - a single mutable borrow excludes other borrows in same scope
    let mut personnel2 = personnel1.clone();
    let mut personnel3 = personnel2.clone();
    println!("{:?}", add_employee("Add Bob to Engineering", &mut personnel2)); 
    println!("{:?}", add_employee("432Add Bob to Engineering", &mut personnel3));
   
    // println!("{}", get_personnel_by_dept("Engineering", &personnel));
    // println!("{}", get_personnel_by_dept("Marketing", &personnel));
    // println!("{}", get_personnel_by_dept("", &personnel));
    // println!("{}", get_personnel_by_dept("123ineering",&personnel));
    // println!("{}", get_all_personnel(&personnel));


}

}