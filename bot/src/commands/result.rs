use amizone::api::client::UserClient;
use amizone::api::types::{ExamResultRecord, OverallResult, go_amizone::server::proto::v1::{Score, Credits}};

use crate::{CommandResult, Context, Result};

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

    let (course_results, overall_result) = match semester {
        Some(sem) => client.get_exam_result(sem).await?,
        None => client.get_current_exam_result().await?,
    };

    let overall_result_vec = overall_result.into_iter()
        .map(|semester| semester.into())
        .collect::<Vec<OverallResult>>();

    let mut message = format!("**Semester Result**\n\n```");
    
    for sem_result in overall_result_vec {
        
        let sem_num = match sem_result.semester {
            Some(ref sem_ref) => sem_ref.semester_ref.as_str(),
            None => ""
        };

        let sgpa = sem_result.semester_grade_point_average;

        let cgpa = sem_result.cumulative_grade_point_average;

        message.push_str(&format!("Semester: {}\n", sem_num));
        message.push_str(&format!("SGPA: {}\n", &sgpa));
        message.push_str(&format!("CGPA: {}\n\n", &cgpa));
    }
    message.push_str("```");
    
    ctx.say(message).await?;
    Ok(())
    
    // let course_wise = course_result.into_iter()
    // .map(|course| course.into())
    // .collect::<Vec<ExamResultRecord>>();
    
    // let course_names = course_wise
    //     .iter()
    //     .map(|course| match course.course{
    //         Some(ref course_ref) => course_ref.name.as_str(),
    //         None => "",
    //     })
    //     .collect::<Vec<&str>>();





    // let course_scores = course_wise
    //     .iter()
    //     .map(|course| match course.score{
    //         Some(ref score) => (score.max, score.grade.as_str(), score.grade_point),
    //         None => (-1, "", -1)
    //     })
    //     .collect::<Vec<(i32, &str, i32)>>();

    
}
fn result_help() -> String {
    RESULT_HELP.into()
}

