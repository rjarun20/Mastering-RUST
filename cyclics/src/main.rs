mod enrollment;

use enrollment::{Student, Course, Platform};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Create a new student
    let student = Rc::new(RefCell::new(Student::new("Alice")));

    // Create new courses
    let math_course = Rc::new(RefCell::new(Course::new("Math")));
    let science_course = Rc::new(RefCell::new(Course::new("Science")));

    // Enroll the student in the courses
    student.borrow_mut().enroll(math_course.borrow().name.clone());
    student.borrow_mut().enroll(science_course.borrow().name.clone());

    // Add student to courses
    math_course.borrow_mut().add_student(student.borrow().name.clone());
    science_course.borrow_mut().add_student(student.borrow().name.clone());

    // Print the results
    println!("Student {} is enrolled in {} course(s).", student.borrow().name, student.borrow().courses().len());
    println!("Course {} has {} student(s) enrolled.", math_course.borrow().name, math_course.borrow().students().len());
    println!("Course {} has {} student(s) enrolled.", science_course.borrow().name, science_course.borrow().students().len());

    // Create a new student and course
    let student = Student { name: String::from("Alice"), courses: Vec::new() };
    let course = Course { name: String::from("Math"), students: Vec::new() };

    // Create a platform and enroll the student in the course
    let mut platform = Platform::new();
    platform.enroll(&student, &course);

    // Print the results
    println!("Student {} is enrolled in the course {}.", student.name, course.name);
}