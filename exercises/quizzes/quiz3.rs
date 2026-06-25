enum Grade {
    Numeric(f32),
    Alphabetical(String),
}

struct ReportCard {
    grade: Grade,
    student_name: String,
    student_age: u8,
}

impl ReportCard {
    fn print(&self) -> String {
        let grade_str = match &self.grade {
            Grade::Numeric(val) => val.to_string(),
            Grade::Alphabetical(val) => val.clone(),
        };
        format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, grade_str)
    }
}

fn main() {
    // Example usage
    let num_grade = ReportCard {
        grade: Grade::Numeric(2.1),
        student_name: "Tom Wriggle".to_string(),
        student_age: 12,
    };
    println!("{}", num_grade.print());

    let alpha_grade = ReportCard {
        grade: Grade::Alphabetical("A+".to_string()),
        student_name: "Gary Plotter".to_string(),
        student_age: 11,
    };
    println!("{}", alpha_grade.print());
}
// Tests can be added similarly, creating ReportCard instances with appropriate Grade variants.