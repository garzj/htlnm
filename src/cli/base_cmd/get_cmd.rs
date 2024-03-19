use clap::{Args, Subcommand};

use crate::api::Api;

use super::print_data;

#[derive(Args)]
pub struct GetCommand {
    /// A handlebars expression defining the output format
    #[arg(short, long)]
    format: Option<String>,

    #[command(subcommand)]
    command: GetCommands,
}

#[derive(Subcommand)]
pub enum GetCommands {
    /// Get the login response
    Login,
    /// Fetch the API hit count
    HitCount,
    /// Fetch the authed student
    Student,
    /// Fetch a list of all classes
    Classes,
    /// Fetch an assessment by id
    Assessment { id: i32 },
    /// Filter assessments
    Assessments {
        #[arg(short, long)]
        class_name: Option<String>,
        #[arg(short, long)]
        subject: Option<String>,
    },
    /// Fetch an assessment's grade
    Grade { assessment_id: i32 },
    /// Filter subjects
    Subjects {
        #[arg(short, long)]
        early_warnings: bool,
    },
    /// Fetch an early warning by id
    EarlyWarning { id: i32 },
    /// Filter the student's early warnings (by subject)
    EarlyWarnings {
        #[arg(short, long)]
        subject: Option<String>,
    },
    /// Fetch early warning settings
    EarlyWarningSettings,
    /// Fetch absences
    Absences,
}

impl GetCommand {
    pub fn run(&self, api: &Api) -> anyhow::Result<()> {
        match self.command {
            GetCommands::Login => print_data(&self.format, api.get_login_data()?),
            GetCommands::HitCount => print_data(&self.format, &api.get_hitcount()?),
            GetCommands::Student => print_data(&self.format, &api.get_student()?),
            GetCommands::Classes => print_data(&self.format, &api.get_classes()?),
            GetCommands::Assessment { id } => print_data(&self.format, &api.get_assessment(id)?),
            GetCommands::Assessments {
                ref class_name,
                ref subject,
            } => print_data(&self.format, &api.get_assessments(class_name, subject)?),
            GetCommands::Grade { assessment_id } => {
                print_data(&self.format, &api.get_grade(assessment_id)?)
            }
            GetCommands::Subjects { early_warnings } => {
                print_data(&self.format, &api.get_subjects(early_warnings)?)
            }
            GetCommands::EarlyWarning { id } => {
                print_data(&self.format, &api.get_early_warning(id)?)
            }
            GetCommands::EarlyWarnings { ref subject } => {
                print_data(&self.format, &api.get_early_warnings(subject)?)
            }
            GetCommands::EarlyWarningSettings => {
                print_data(&self.format, &api.get_early_warning_settings()?)
            }
            GetCommands::Absences => print_data(&self.format, &api.get_absences()?),
        }
    }
}
