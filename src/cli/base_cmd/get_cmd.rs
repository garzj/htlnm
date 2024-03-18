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
    Assessment { id: String },
    /// Filter assessments by class (or subject)
    Assessments(AssessmentsCommand),
    /// Fetch a grade by assessment id
    Grade { assessment_id: String },
}

#[derive(Args)]
pub struct AssessmentsCommand {
    class: String,
    subject: Option<String>,
}

impl GetCommand {
    pub fn run(&self, api: &Api) -> anyhow::Result<()> {
        match self.command {
            GetCommands::Login => self.print_data(api.get_login_data()?),
            GetCommands::HitCount => self.print_data(&api.get_hitcount()?),
            GetCommands::Student => self.print_data(&api.get_student()?),
            GetCommands::Classes => self.print_data(&api.get_classes()?),
            GetCommands::Assessment { ref id } => self.print_data(&api.get_assessment(id)?),
            GetCommands::Assessments(ref ass_cmd) => {
                if let Some(ref subject) = ass_cmd.subject {
                    self.print_data(&api.get_subject_assessments(&ass_cmd.class, subject)?)
                } else {
                    self.print_data(&api.get_assessments(&ass_cmd.class)?)
                }
            }
            GetCommands::Grade { ref assessment_id } => {
                self.print_data(&api.get_grade(assessment_id)?)
            }
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
