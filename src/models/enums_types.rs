use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Genderenum"]
pub enum Genderenum {
    Male,
    Female,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Ratingenum"]
pub enum Ratingenum {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Weightenum"]
pub enum Weightenum {
    Kg,
    Lb,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Heightenum"]
pub enum Heightenum {
    Cm,
    FtIn,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Difficultyenum"]
pub enum Difficultyenum {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Musclegroupenum"]
pub enum Musclegroupenum {
    Chest,
    LowerBack,
    UpperBack,
    Lats,
    Traps,
    Quads,
    Hamstrings,
    Calves,
    Shoulders,
    Forearms,
    Biceps,
    Triceps,
    Abs,
    Obliques,
    Cardio,
    Compound,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Visibilityenum"]
pub enum Visibilityenum {
    Public,
    Private,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Workoutdayenum"]
pub enum Workoutdayenum {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Workoutgoalenum"]
pub enum Workoutgoalenum {
    BuildMuscle,
    BuildStrength,
    LoseWeight,
    ImproveEndurance,
    MaintainFitness,
    ToneBody,
    TrainForEvent,
    ImproveFlexibility,
    ImproveHealth,
    ImproveBalance,
    ImproveCoordination,
    ImprovePower,
    ImproveSpeed,
    Custom,
}

#[derive(diesel_derive_enum::DbEnum,Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Equipmentenum"]
pub enum Equipmentenum {
    Barbell,
    Dumbbell,
    Cable,
    Machine,
    Kettlebell,
    Bands,
    Bar,
    Rings,
    EzBar,
    SmithMachine,
    Bodyweight,
    Other,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Fatiguelevelenum"]
pub enum Fatiguelevelenum {
    VeryLight,
    Light,
    Moderate,
    Hard,
    VeryHard,
}
