// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


use std::fmt::Display;

#[derive(Debug)]
pub enum Letter {
    A, B, C, D, E, F
}

impl Display for  Letter{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum LetterSign {
    Plus, Minus, None
}

impl LetterSign {
    fn to_symbol(&self) -> String {
        match self {
            LetterSign::Plus => "+".to_string(),
            LetterSign::Minus => "-".to_string(),
            LetterSign::None => String::new()
        }
    }
}

impl Display for LetterSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_symbol())
    }
}
struct LetterGrade {
    letter: Letter,
    sign: LetterSign
}

impl Display for LetterGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.sign)
    }
}

pub trait Grade: Display { }
impl Grade for f32 { }
impl Grade for LetterGrade { }

pub struct ReportCard<T: Grade> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: Grade> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: LetterGrade {
                letter: Letter::A,
                sign: LetterSign::Plus
            },
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
