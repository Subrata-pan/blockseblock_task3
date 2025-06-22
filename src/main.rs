use std::fs::File;
use std::io::BufWriter;
use printpdf::*;
use std::io::{self, Write};

struct Student {
    name: String,
    total_marks: f32,
    num_subjects: u32,
}

impl Student {
    fn average(&self) -> f32 {
        self.total_marks / self.num_subjects as f32
    }

    fn grade(&self) -> char {
        let avg = self.average();
        match avg {
            90.0..=100.0 => 'A',
            75.0..=89.99 => 'B',
            60.0..=74.99 => 'C',
            _ => 'D',
        }
    }

    fn generate_report_card(&self) {
        let (doc, page1, layer1) = PdfDocument::new(
            "Report Card",
            Mm(210.0),
            Mm(297.0),
            "Layer 1",
        );
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

        let text = format!(
            "Report Card\n\nStudent Name: {}\nTotal Marks: {}\nSubjects: {}\nAverage: {:.2}\nGrade: {}",
            self.name,
            self.total_marks,
            self.num_subjects,
            self.average(),
            self.grade()
        );

        current_layer.use_text(text, 12.0, Mm(20.0), Mm(270.0), &font);

        doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap())).unwrap();
    }
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    print!("Enter student name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Enter total marks: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut total_marks).unwrap();

    print!("Enter number of subjects: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num_subjects).unwrap();

    let student = Student {
        name: name.trim().to_string(),
        total_marks: total_marks.trim().parse().unwrap(),
        num_subjects: num_subjects.trim().parse().unwrap(),
    };

    student.generate_report_card();
    println!("Report card generated as PDF!");
}
