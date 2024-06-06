// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub struct ReportCard<G> {
    pub grade: G,
    pub student_name: String,
    pub student_age: u8,
}

pub trait Grade {
    fn as_str(&self) -> &str;
}

impl Grade for f32 {
    fn as_str(&self) -> &str {
        &self.to_string()
    }
}

impl<'a> Grade for &'a str {
    fn as_str(&self) -> &str {
        self
    }
}

impl<G> ReportCard<G>
where
    G: Grade,
{
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, self.grade.as_str())
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
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
fn main (){}
///explication
/// Le problème réside dans le fait que la structure `ReportCard` ne prend actuellement en charge que les notes numériques, représentées par des nombres à virgule flottante (`f32`). Cependant, l'école émet également des notes alphabétiques (par exemple, "A+" à "F-") et doit être capable d'imprimer les deux types de bulletins !
///Pour prendre en charge les notes numériques et alphabétiques, nous allons utiliser les génériques et les traits de Rust. Nous allons créer un trait Grade qui sera implémenté pour les types f32 et &str, ce qui nous permettra de stocker soit des notes numériques soit des notes alphabétiques dans la structure ReportCard
/// Nous avons choisie cette solution car elle permet une extension facile pour prendre en charge d'autres types de notes à l'avenir, tout en maintenant la flexibilité et la lisibilité du code.
