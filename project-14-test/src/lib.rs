mod cardio;

mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {}", NUTRITIONIST);
    }
}
mod weightlifting;

use cardio::{CardioTool, Exercise as CardioExercise};
use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        cardio::ask_about_program();
        diet::ask_about_program();
        weightlifting::ask_about_program();

        return Self {
            cardio:{CardioExercise::new(String::from("20260413"),CardioTool::Bike,35)},
            weightlifting:{WeightliftingExercise::new(String::from("bibiLabu"),41)},
        };
    }
}
