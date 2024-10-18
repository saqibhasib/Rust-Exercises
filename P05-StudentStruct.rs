use std::fmt::{self, Formatter, Display};

#[derive(Clone)]
struct Student {
    name: String,
    age: i32,
    grade: f32
}

#[derive(Clone)]
struct StudentVec(Vec<Student>);

impl Display for Student {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "name: {}, \nage: {}, \ngrade: {}", self.name, self.age, self.grade)
    }
}

impl Display for StudentVec {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let _ = write!(f, "[\n");
        for s in &self.0 {
            let __ = write!(f, "\tname: {}, age: {}, grade: {}\n", s.name, s.age, s.grade);
        }
        write!(f, "]\n")
    }
}


// Stack Overflow: https://stackoverflow.com/questions/28800121/what-do-i-have-to-do-to-solve-a-use-of-moved-value-error

// fn students_over_age_(input: &[Student], _age: i32) -> Vec<Student>{
//     let mut result = Vec::<Student>::new();
//     for i in input {
//         if i.age > _age {
//             result.push(Student{
//                 name: i.name.clone(),
//                 age: i.age,
//                 grade: i.grade
//             });
//             // result.push(*i.clone());
//         }
//     }
//     return result;
// }

fn students_over_age_(input: Vec<Student>, _age: i32) -> Vec<Student>{
    let mut result = Vec::<Student>::new();
    for i in input {
        if i.age > _age {
            result.push(i);
        }
    }
    return result;
}

fn main() {
    let mut student_vec = Vec::<Student>::new();
    student_vec.push( Student{
        name: "rob".to_string(), 
        age: 22, 
        grade: 3.3
    });
    student_vec.push( Student{
        name: "anna".to_string(), 
        age: 17, 
        grade: 3.2
    });
    student_vec.push( Student{
        name: "jack".to_string(), 
        age: 18, 
        grade: 2.3
    });
    student_vec.push( Student{
        name: "rue".to_string(), 
        age: 28, 
        grade: 2.9
    });

    student_vec = students_over_age_(student_vec, 18);

    let result_vec_struct = StudentVec(student_vec);

    println!("{}", result_vec_struct);
}