// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
#[derive(Debug)]
enum Option<T>{
    Some(T),
    None
}

struct StudentInfo {
    name: String,
    locker: Option<i32>
}

fn main() {
    let students = vec![
        StudentInfo {
            name: "Stefan".to_owned(),
            locker: Option::Some(13)
        },
        StudentInfo {
            name: "Ivan".to_owned(),
            locker: Option::None
        },
        StudentInfo {
            name: "Valena".to_owned(),
            locker: Option::Some(14)
        },
    ];
    let variant1 = false;
    if variant1 {
        for student in students {
            println!("Name: {:?}, Locker: {:?}", student.name, student.locker)
        }
    } else {
        for student in students {
            println!("Student's name is: {:?}", student.name);
            match student.locker {
                Option::Some(num) => println!("Student locket number: {:?}", num),
                Option::None => println!("Student doesn't have a locker")
            }
        }
        
    }

}
