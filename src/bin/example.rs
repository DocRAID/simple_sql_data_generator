use requestty::{DefaultSeparator, Question};

fn main() {
    let questions = vec![
        Question::select("theme")
            .message("What do you want to do?")
            .choices(vec![
                "Order a pizza".into(),
                "Make a reservation".into(),
                DefaultSeparator,
                "Ask for opening hours".into(),
                "Contact support".into(),
                "Talk to the receptionist".into(),
            ])
            .build()
    ];

    println!("{:#?}", requestty::prompt(questions));
}