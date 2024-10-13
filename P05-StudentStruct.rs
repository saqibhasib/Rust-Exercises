use std::fmt::{self, Formatter, Display};

struct Student {
    name: String,
    age: i32,
    grade: f32
}

struct StudentVec(Vec<Student>);

impl Display for Student {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "name: {}, \nage: {}, \ngrade: {}", self.name, self.age, self.grade)
    }
}

impl Display for StudentVec {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[\n");
        for s in &self.0 {
            write!(f, "\tname: {}, age: {}, grade: {}\n", s.name, s.age, s.grade);
        }
        write!(f, "]\n")
    }
}

fn students_over_age_(mut input: Vec<Student>, _age: i32) -> Vec<Student>{
    input.push( Student{
        name: "ryan".to_string(), 
        age: 32, 
        grade: 3.7
    });
    return Vec::<Student>::new();
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
    // student_vec.push( Student{
    //     name: "jack", 
    //     age: 18, 
    //     grade: 2.3
    // });

    let mut student_vec_struct = StudentVec(student_vec);

    // let _student_vec = students_over_age_(student_vec, 18);

    println!("{}", student_vec_struct);
}