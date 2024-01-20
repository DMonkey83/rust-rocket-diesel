use diesel::deserialize::FromSql;
use std::io::Write;
use std::str::FromStr;
use diesel::pg::PgValue;
use diesel::pg::Pg;
use diesel::serialize::ToSql;
use diesel::{expression::AsExpression, deserialize::FromSqlRow};
use diesel::sql_types::Text;
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

#[derive(AsExpression, Debug, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub enum RoleCode {
    Admin,
    Editor,
    User,
}

impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"Admin" => Ok(RoleCode::Admin),
            b"Editor" => Ok(RoleCode::Editor),
            b"User" => Ok(RoleCode::User),
            _ => Ok(RoleCode::User),
        }
    }

}


impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"Admin")?,
            RoleCode::Editor => out.write_all(b"Editor")?,
            RoleCode::User => out.write_all(b"User")?,
        };
        Ok(diesel::serialize::IsNull::No)
    }
}


impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(RoleCode::Admin),
            "Editor" => Ok(RoleCode::Editor),
            "User" => Ok(RoleCode::User),
            _ => Err(()),
        }
    }
}

impl ToString for RoleCode {
    fn to_string(&self) -> String {
        match self {
            RoleCode::Admin => String::from("Admin"),
            RoleCode::Editor => String::from("Editor"),
            RoleCode::User => String::from("User"),
        }
    }
}

