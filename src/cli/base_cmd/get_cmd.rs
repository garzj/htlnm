use clap::{Args, Subcommand};
use handlebars::Handlebars;
use serde::Serialize;

use crate::api::Api;

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
            GetCommands::Login => self.print_data(api.get_login_data()?),
            GetCommands::HitCount => self.print_data(&api.get_hitcount()?),
            GetCommands::Student => self.print_data(&api.get_student()?),
            GetCommands::Classes => self.print_data(&api.get_classes()?),
            GetCommands::Assessment { id } => self.print_data(&api.get_assessment(id)?),
            GetCommands::Assessments {
                ref class_name,
                ref subject,
            } => self.print_data(&api.get_assessments(class_name, subject)?),
            GetCommands::Grade { assessment_id } => self.print_data(&api.get_grade(assessment_id)?),
            GetCommands::Subjects { early_warnings } => {
                self.print_data(&api.get_subjects(early_warnings)?)
            }
            GetCommands::EarlyWarning { id } => self.print_data(&api.get_early_warning(id)?),
            GetCommands::EarlyWarnings { ref subject } => {
                self.print_data(&api.get_early_warnings(subject)?)
            }
            GetCommands::EarlyWarningSettings => {
                self.print_data(&api.get_early_warning_settings()?)
            }
            GetCommands::Absences => self.print_data(&api.get_absences()?),
        }
    }

    fn print_data<T>(&self, data: &T) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        println!(
            "{}",
            self.format.as_ref().map_or_else(
                || anyhow::Ok(serde_json::to_string_pretty(data)?),
                |format| anyhow::Ok(Handlebars::new().render_template(&format, data)?)
            )?
        );
        Ok(())
    }
}
