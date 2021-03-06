//! Output format

use schemars::JsonSchema;
use serde::Serialize;
use std::path::PathBuf;
use tmc_client::{
    ClientUpdateData, Course, CourseData, CourseDetails, CourseExercise, ExerciseDetails,
    NewSubmission, Organization, Review, RunResult, StyleValidationResult, Submission,
    SubmissionFeedbackResponse, SubmissionFinished, Token, UpdateResult,
};
use tmc_langs_util::{
    progress_reporter::StatusUpdate, task_executor::RefreshData, ExerciseDesc,
    ExercisePackagingConfiguration,
};

use crate::config::{ConfigValue, TmcConfig};

/// The format for all messages written to stdout by the CLI
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "output-kind")]
pub enum Output {
    /// Data that is output at the end of a command.
    OutputData(OutputData),
    /// Status update output as a command progresses.
    StatusUpdate(StatusUpdateData),
    /// Additional warnings, such as for an outdated Python dependency.
    Warnings(Warnings),
}

impl Output {
    pub fn finished_with_data(message: impl Into<String>, data: impl Into<Option<Data>>) -> Self {
        Self::OutputData(OutputData {
            status: Status::Finished,
            message: message.into(),
            result: OutputResult::ExecutedCommand,
            data: data.into(),
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OutputData {
    pub status: Status,
    pub message: String,
    pub result: OutputResult,
    pub data: Option<Data>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "output-data-kind", content = "output-data")]
pub enum Data {
    Error { kind: Kind, trace: Vec<String> },
    Validation(StyleValidationResult),
    FreeDiskSpace(u64),
    AvailablePoints(Vec<String>),
    Exercises(Vec<PathBuf>),
    ExercisePackagingConfiguration(ExercisePackagingConfiguration),
    LocalExercises(Vec<LocalExercise>),
    RefreshResult(RefreshData),
    TestResult(RunResult),
    ExerciseDesc(ExerciseDesc),
    UpdatedExercises(Vec<UpdatedExercise>),
    ExerciseDownload(DownloadOrUpdateCourseExercisesResult),
    CombinedCourseData(Box<CombinedCourseData>),
    CourseDetails(CourseDetails),
    CourseExercises(Vec<CourseExercise>),
    CourseData(CourseData),
    Courses(Vec<Course>),
    ExerciseDetails(ExerciseDetails),
    Submissions(Vec<Submission>),
    UpdateResult(UpdateResult),
    Organization(Organization),
    Organizations(Vec<Organization>),
    Reviews(Vec<Review>),
    Token(Token),
    NewSubmission(NewSubmission),
    SubmissionFeedbackResponse(SubmissionFeedbackResponse),
    SubmissionFinished(SubmissionFinished),
    ConfigValue(ConfigValue<'static>),
    TmcConfig(TmcConfig),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "update-data-kind")]
pub enum StatusUpdateData {
    ClientUpdateData(StatusUpdate<ClientUpdateData>),
    None(StatusUpdate<()>),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// The command was ran without fatal errors
    Finished,
    /// An unexpected issue occurred during the command
    Crashed,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputResult {
    LoggedIn,
    LoggedOut,
    NotLoggedIn,
    Error,
    ExecutedCommand,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// For all other errors
    Generic,
    /// 403 from server
    Forbidden,
    /// Not logged in, detected either by no token or 401 from server
    NotLoggedIn,
    /// Failed to connect to the TMC server, likely due to no internet connection
    ConnectionError,
    /// Client out of date
    ObsoleteClient,
    /// Invalid token
    InvalidToken,
    /// Failed to download some or all exercises
    FailedExerciseDownload {
        completed: Vec<DownloadOrUpdateCourseExercise>,
        skipped: Vec<DownloadOrUpdateCourseExercise>,
        failed: Vec<(DownloadOrUpdateCourseExercise, Vec<String>)>,
    },
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct CombinedCourseData {
    pub details: CourseDetails,
    pub exercises: Vec<CourseExercise>,
    pub settings: CourseData,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct DownloadOrUpdateCourseExercisesResult {
    pub downloaded: Vec<DownloadOrUpdateCourseExercise>,
    pub skipped: Vec<DownloadOrUpdateCourseExercise>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DownloadOrUpdateCourseExercise {
    pub course_slug: String,
    pub exercise_slug: String,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LocalExercise {
    pub exercise_slug: String,
    pub exercise_path: PathBuf,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct UpdatedExercise {
    pub id: usize,
}

#[derive(Debug, Serialize)]
pub struct DownloadTarget {
    pub id: usize,
    pub path: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct Warnings {
    warnings: Vec<String>,
}

impl Warnings {
    pub fn from_error_list(warnings: &[anyhow::Error]) -> Self {
        Self {
            warnings: warnings.iter().map(|w| w.to_string()).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn read_api_file(filename: &str) -> String {
        std::fs::read_to_string(std::path::Path::new("api").join(filename)).unwrap()
    }

    #[test]
    fn output_data_none() {
        let output_data = Output::OutputData(OutputData {
            status: Status::Finished,
            message: "output with no data".to_string(),
            result: OutputResult::ExecutedCommand,
            data: None,
        });
        let actual = serde_json::to_string_pretty(&output_data).unwrap();
        let expected = read_api_file("output-data-none.json");
        assert_eq!(actual, expected);
    }

    #[test]
    fn output_data_error() {
        let output_data = Output::OutputData(OutputData {
            status: Status::Finished,
            message: "errored!".to_string(),
            result: OutputResult::Error,
            data: Some(Data::Error {
                kind: Kind::Generic,
                trace: vec!["trace 1".to_string(), "trace 2".to_string()],
            }),
        });
        let actual = serde_json::to_string_pretty(&output_data).unwrap();
        let expected = read_api_file("output-data-error.json");
        assert_eq!(actual, expected);
    }

    #[test]
    fn output_data_dl() {
        let output_data = Output::OutputData(OutputData {
            status: Status::Finished,
            message: "downloaded things".to_string(),
            result: OutputResult::ExecutedCommand,
            data: Some(Data::ExerciseDownload(
                DownloadOrUpdateCourseExercisesResult {
                    downloaded: vec![
                        DownloadOrUpdateCourseExercise {
                            course_slug: "some course".to_string(),
                            exercise_slug: "some exercise".to_string(),
                            path: PathBuf::from("some path"),
                        },
                        DownloadOrUpdateCourseExercise {
                            course_slug: "some course".to_string(),
                            exercise_slug: "another exercise".to_string(),
                            path: PathBuf::from("another path"),
                        },
                    ],
                    skipped: vec![DownloadOrUpdateCourseExercise {
                        course_slug: "another course".to_string(),
                        exercise_slug: "some skipped exercise".to_string(),
                        path: PathBuf::from("third path"),
                    }],
                },
            )),
        });
        let actual = serde_json::to_string_pretty(&output_data).unwrap();
        let expected = read_api_file("output-data-download-or-update.json");
        assert_eq!(actual, expected);
    }

    #[test]
    fn status_update() {
        let status_update =
            Output::StatusUpdate(StatusUpdateData::ClientUpdateData(StatusUpdate {
                data: Some(ClientUpdateData::ExerciseDownload {
                    id: 1234,
                    path: PathBuf::from("some path"),
                }),
                finished: false,
                message: "doing things...".to_string(),
                percent_done: 33.3333,
                time: Some(2000),
            }));
        let actual = serde_json::to_string_pretty(&status_update).unwrap();
        let expected = read_api_file("status-update.json");
        assert_eq!(actual, expected);
    }

    #[test]
    fn warnings() {
        let status_update = Output::Warnings(Warnings {
            warnings: vec!["warning 1".to_string(), "warning 2".to_string()],
        });
        let actual = serde_json::to_string_pretty(&status_update).unwrap();
        let expected = read_api_file("warnings.json");
        assert_eq!(actual, expected);
    }
}
