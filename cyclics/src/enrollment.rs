pub struct Student {
    pub name: String,
    pub courses: Vec<String>,
}

impl Student {
    pub fn new(name: &str) -> Self {
        Student {
            name: name.to_string(),
            courses: Vec::new(),
        }
    }

    pub fn enroll(&mut self, course: String) {
        self.courses.push(course);
    }

    pub fn courses(&self) -> &Vec<String> {
        &self.courses
    }
}

pub struct Course {
    pub name: String,
    pub students: Vec<String>,
}

impl Course {
    pub fn new(name: &str) -> Self {
        Course {
            name: name.to_string(),
            students: Vec::new(),
        }
    }

    pub fn add_student(&mut self, student: String) {
        self.students.push(student);
    }

    pub fn students(&self) -> &Vec<String> {
        &self.students
    }
}

pub struct Enrollment<'a> {
    pub student: &'a Student,
    pub course: &'a Course,
}

impl<'a> Enrollment<'a> {
    pub fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }

    pub fn student(&self) -> &Student {
        self.student
    }

    pub fn course(&self) -> &Course {
        self.course
    }
}

pub struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    pub fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new(),
        }
    }

    pub fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        let enrollment = Enrollment::new(student, course);
        self.enrollments.push(enrollment);
    }
}