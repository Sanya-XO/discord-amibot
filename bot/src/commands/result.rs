use std::ops::Deref;

use amizone::api::{client::UserClient, types::{ExamResultRecord, OverallResult, Course, Semester, go_amizone::server::proto::v1::{Score, Credits}}};
use poise::serenity_prelude::CreateEmbed;

use crate::{CommandResult, Context, Result};
use crate::util;
use log::info;

static RESULT_HELP: &str = "/results - Retrieve your examination results.\n\n\
Usage: /results\n\n\
Example:\n\
/results\n\n\
Note: This command fetches and displays your examinations results.\
If you have any scheduled exams, it will show the course name, course code, date, and exam mode.\
If you don't have any upcoming exams, it will not return any information.";

/// Retrieves your datesheet for upcoming examination
#[poise::command(prefix_command, slash_command, help_text_fn = "result_help")]
pub async fn results(ctx: Context<'_>,
    #[description = "Semester number"] semester: Option<usize>) -> CommandResult {
    
    ctx.defer().await?;
    let mut invocation_data = ctx.invocation_data::<Result<UserClient>>().await.unwrap();

    let client = invocation_data.as_mut()?;

    //let (course_result, overall_result) = client.get_current_exam_result().await?;
    let (course_result, overall_result) = match semester {
        Some(sem) => client.get_exam_result(sem).await?,
        None => client.get_current_exam_result().await?,
    };

    let course_wise = course_result.into_iter()
    .map(|course| course.into())
    .collect::<Vec<ExamResultRecord>>();
    
    let course_names = course_wise
        .iter()
        .map(|course| match course.course{
            Some(ref course_ref) => course_ref.name.as_str(),
            None => "",
        })
        .collect::<Vec<&str>>();

    let overall_semester_wise = overall_result.into_iter()
        .map(|semester| semester.into())
        .collect::<Vec<OverallResult>>();



    // let course_scores = course_wise
    //     .iter()
    //     .map(|course| match course.score{
    //         Some(ref score) => (score.max, score.grade.as_str(), score.grade_point),
    //         None => (0, "nil", 0)
    //     })
    //     .collect::<Vec<(i32, &str, i32)>>();
    

    Ok(())
    
}
fn result_help() -> String {
    RESULT_HELP.into()
}

