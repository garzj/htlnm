use std::collections::HashMap;

use clap::Args;
use serde_json::{json, Value};

use crate::api::Api;

use super::print_data;

#[derive(Args)]
pub struct DumpCommand {
    /// A handlebars expression defining the output format
    #[arg(short, long)]
    format: Option<String>,
}

fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }
            return;
        }
    }
    *a = b;
}

impl DumpCommand {
    pub fn run(&self, api: &Api) -> anyhow::Result<()> {
        print_data(&self.format, &self.dump(api)?)
    }

    fn dump(&self, api: &Api) -> anyhow::Result<Value> {
        let absences = api.get_absences()?;
        let absences = json!({
            "excused": absences.excused,
            "unexcused": absences.unexcused,
            "open": absences.open
        });

        let mut assessment_vals = HashMap::<String, Vec<Value>>::new();
        let assessments = api.get_assessments(&None, &None)?;
        for assessment in assessments {
            let grade = api.get_grade(assessment.id)?;
            let assessment_val = json!({
                "id": assessment.id,
                "date": assessment.date,
                "type": assessment.r#type,
                "comment": assessment.comment,
                "maxPoints": assessment.max_points,
                "teacherId": assessment.teacher_id,
                "grade": grade
            });
            let subject = assessment.subject.expect("Subject name cannot be empty.");
            let assessment_entry = assessment_vals.entry(subject).or_insert_with(|| Vec::new());
            assessment_entry.push(assessment_val);
        }

        let subjects = api.get_subjects(false)?;
        let subjects = subjects
            .into_iter()
            .map(|subject| {
                let mut subject_val = serde_json::to_value(&subject)?;
                merge(
                    &mut subject_val,
                    json!({
                        "assessments": assessment_vals.get(&subject.name)
                    }),
                );
                anyhow::Ok(subject_val)
            })
            .try_fold(Vec::new(), |mut v, subject| {
                v.push(subject?);
                anyhow::Ok(v)
            })?;

        let mut student = serde_json::to_value(api.get_student()?)?;
        merge(
            &mut student,
            json!({
                "absences": absences,
                "subjects": subjects
            }),
        );
        Ok(student)
    }
}
