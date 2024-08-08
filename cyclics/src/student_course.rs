use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>,
}

impl Student {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            courses: Vec::new(),
        }
    }

    pub fn enroll(&mut self, course: Rc<RefCell<Course>>) {
        self.courses.push(course.clone());
        course.borrow_mut().add_student(Rc::new(RefCell::new(self.clone())));
    }

    // Getter for name
    pub fn name(&self) -> &str {
        &self.name
    }

    // Getter for courses
    pub fn courses(&self) -> &Vec<Rc<RefCell<Course>>> {
        &self.courses
    }
}

pub struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>,
}

impl Course {
    pub fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new(),
        }
    }

    pub fn add_student(&mut self, student: Rc<RefCell<Student>>) {
        self.students.push(student);
    }

    // Getter for name
    pub fn name(&self) -> &str {
        &self.name
    }

    // Getter for students
    pub fn students(&self) -> &Vec<Rc<RefCell<Student>>> {
        &self.students
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_enrollment() {
        let student = Rc::new(RefCell::new(Student::new("Alice")));
        let course = Rc::new(RefCell::new(Course::new("Math")));

        student.borrow_mut().enroll(course.clone());

        assert_eq!(student.borrow().courses().len(), 1);
        assert_eq!(course.borrow().students().len(), 1);
    }
}