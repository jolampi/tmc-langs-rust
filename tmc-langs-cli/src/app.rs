//! Create clap app

use clap::{App, Arg, SubCommand};

pub fn create_app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))

        .subcommand(SubCommand::with_name("checkstyle")
            .about("Check code style for the exercise.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the project resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the file where the check results will be written. Overwritten if it already exists.")
                .long("output-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("locale")
                .help("Language as a three letter ISO 639-3 code, e.g. 'eng' or 'fin'.")
                .long("locale")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("compress-project")
            .about("Compress target exercise into a ZIP.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the exercise resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the output archive. Overwritten if it already exists.")
                .long("output-path")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("extract-project")
            .about("Extract an exercise archive.")
            .arg(Arg::with_name("archive-path")
                .help("Path to the archive.")
                .long("archive-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the directory where the archive will be extracted.")
                .long("output-path")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("prepare-solutions")
            .about("Prepare a presentable solution from the original.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the exercise resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the directory where the processed files will be copied.")
                .long("output-path")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("prepare-stubs")
            .about("Prepare a stub exercise from the original.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the exercise resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the directory where the processed files will be copied.")
                .long("output-path")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("prepare-submission")
            .about("Prepares from submission and solution project for which the tests can be run in sandbox.")
            .arg(Arg::with_name("submission-path")
                .help("Path to the zip archive to be submitted.")
                .long("submission-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the resulting archive.")
                .long("output-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("tmc-param")
                .help("A key-value pair in the form key=value to be written into .tmcparams. If multiple pairs with the same key are given, the values are interpreted as an array.")
                .long("tmc-param")
                .takes_value(true)
                .multiple(true))
            .arg(Arg::with_name("clone-path")
                .help("Path to exercise's clone path, where the unmodified test files will be copied from.")
                .long("clone-path")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("output-zip")
                .help("Packages the submission into a zip archive instead of a tar.")
                .long("output-zip"))
            .arg(Arg::with_name("top-level-dir-name")
                .help("If given, the contents in the resulting archive will be nested inside a directory with this name.")
                .long("top-level-dir-name")
                .takes_value(true))
            .arg(Arg::with_name("stub-zip-path")
                .help("If given, the tests will be copied from the stub zip instead, effectively ignoring hidden tests.")
                .long("stub-zip-path")
                .takes_value(true)))

        .subcommand(SubCommand::with_name("run-tests")
            .about("Run the tests for the exercise.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the exercise resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the file where the test results will be written. Overwritten if it already exists.")
                .long("output-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("checkstyle-output-path")
                .help("Runs checkstyle if given. Path to the file where the style results will be written.")
                .long("checkstyle-output-path")
                .takes_value(true)
                .requires("locale"))
            .arg(Arg::with_name("locale")
                .help("Language as a three letter ISO 639-3 code, e.g. 'eng' or 'fin'. Required if checkstyle-output-path is given.")
                .long("locale")
                .takes_value(true)))

        .subcommand(SubCommand::with_name("scan-exercise")
            .about("Produce an exercise description of an exercise directory.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the project resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the file where the scan results will be written. Overwritten if it already exists.")
                .long("output-path")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("find-exercises")
            .about("Produce list of found exercises.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the project resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the file where the search results will be written. Overwritten if it already exists.")
                .long("output-path")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("get-exercise-packaging-configuration")
            .about("Returns configuration of under which folders student and nonstudent files are located.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the exercise resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output-path")
                .help("Path to the file where the configuration will be written. Overwritten if it already exists.")
                .long("output-path")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("clean")
            .about("Clean target directory.")
            .arg(Arg::with_name("exercise-path")
                .help("Path to the directory where the exercise resides.")
                .long("exercise-path")
                .required(true)
                .takes_value(true)))

        .subcommand(SubCommand::with_name("core")
            .about("tmc-core commands. The program will ask for your TMC password through stdin.")
            .arg(Arg::with_name("client-name")
                .help("Name used to differentiate between different TMC clients")
                .long("client-name")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("client-version")
                .help("Client version")
                .long("client-version")
                .required(true)
                .takes_value(true))

            .subcommand(SubCommand::with_name("login")
                .about("Login and store OAuth2 token in config. You can login either by email and password or an access token.")
                .arg(Arg::with_name("email")
                    .help("The email address of your TMC account.")
                    .long("email")
                    .takes_value(true)
                    .required_unless("set-access-token"))
                .arg(Arg::with_name("set-access-token")
                    .help("The OAUTH2 access token that should be used for authentication")
                    .long("set-access-token")
                    .takes_value(true)
                    .required_unless("email"))
                .arg(Arg::with_name("base64")
                    .help("Write password as base64 encoded string.")
                    .long("base64")))

            .subcommand(SubCommand::with_name("logout")
                .about("Logout and remove OAuth2 token from config."))

            .subcommand(SubCommand::with_name("logged-in")
                .about("Check if the CLI is logged in. Prints the access token if so."))

            .subcommand(SubCommand::with_name("get-organizations")
                .about("Get organizations."))

            .subcommand(SubCommand::with_name("get-organization")
                .about("Get organization.")
                .arg(Arg::with_name("organization")
                    .help("Organization slug (e.g. mooc, hy).")
                    .long("organization")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("download-or-update-exercises")
                .about("Download exercise.")
                .arg(Arg::with_name("exercise")
                    .help("An exercise. Takes two values, an exercise id and an exercise path. Multiple exercises can be given.")
                    .long("exercise")
                    .required(true)
                    .takes_value(true)
                    .number_of_values(2)
                    .value_names(&["exercise-id", "exercise-path"])
                    .multiple(true)))

            .subcommand(SubCommand::with_name("get-course-details")
                .about("Get course details.")
                .arg(Arg::with_name("course-id")
                    .help("The ID of the course.")
                    .long("course-id")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("get-courses")
                .about("List courses.")
                .arg(Arg::with_name("organization")
                    .help("Organization slug (e.g. mooc, hy).")
                    .long("organization")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("get-course-settings")
                .about("Get course settings.")
                .arg(Arg::with_name("course-id")
                    .help("The ID of the course.")
                    .long("course-id")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("get-course-exercises")
                .about("Get course exercises.")
                .arg(Arg::with_name("course-id")
                    .help("The ID of the course.")
                    .long("course-id")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("get-exercise-details")
                .about("Get exercise details.")
                .arg(Arg::with_name("exercise-id")
                    .help("The ID of the exercise.")
                    .long("exercise-id")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("paste")
                .about("Send exercise to pastebin with comment.")
                .arg(Arg::with_name("submission-url")
                    .help("The URL where the submission should be posted.")
                    .long("submission-url")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("submission-path")
                    .help("Path to the exercise to be submitted.")
                    .long("submission-path")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("paste-message")
                    .help("Optional message to attach to the paste.")
                    .long("paste-message")
                    .takes_value(true))
                .arg(Arg::with_name("locale")
                    .help("Language as a three letter ISO 639-3 code, e.g. 'eng' or 'fin'.")
                    .long("locale")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("run-checkstyle")
                .about("Run checkstyle.")
                .arg(Arg::with_name("exercise-path")
                    .help("Path to the directory where the exercise resides.")
                    .long("exercise-path")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("locale")
                    .help("Language as a three letter ISO 639-3 code, e.g. 'eng' or 'fin'.")
                    .long("locale")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("run-tests")
                .about("Run tests.")
                .arg(Arg::with_name("exercise-path")
                    .help("Path to the directory where the exercise resides.")
                    .long("exercise-path")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("send-feedback")
                .about("Send feedback.")
                .arg(Arg::with_name("feedback-url")
                    .help("URL where the feedback should be posted.")
                    .long("feedback-url")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("feedback")
                    .help("A feedback answer. Takes two values, a feedback answer id and the answer. Multiple feedback arguments can be given.")
                    .long("feedback")
                    .required(true)
                    .takes_value(true)
                    .number_of_values(2)
                    .value_names(&["feedback-answer-id", "answer"])
                    .multiple(true)))

            .subcommand(SubCommand::with_name("submit")
                .about("Submit exercise. Will block until the submission results are returned.")
                .arg(Arg::with_name("submission-url")
                    .help("URL where the submission should be posted.")
                    .long("submission-url")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("submission-path")
                    .help("Path to the directory where the exercise resides.")
                    .long("submission-path")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("locale")
                    .help("Language as a three letter ISO 639-3 code, e.g. 'eng' or 'fin'.")                    
                    .long("locale")
                    .takes_value(true))
                .arg(Arg::with_name("dont-block")
                    .help("Set to avoid blocking.")
                    .long("dont-block")))

            .subcommand(SubCommand::with_name("wait-for-submission")
                .about("Wait for a submission to finish.")
                .arg(Arg::with_name("submission-url")
                    .help("URL to the submission's status.")
                    .long("submission-url")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("get-exercise-updates")
                .about("Get exercise updates.")
                .arg(Arg::with_name("course-id")
                    .help("The ID of the course")
                    .long("course-id")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("exercise")
                    .help("An exercise. Takes two values, an exercise id and a checksum. Multiple exercises can be given.")
                    .long("exercise")
                    .required(true)
                    .takes_value(true)
                    .number_of_values(2)
                    .value_names(&["exercise-id", "checksum"])
                    .multiple(true)))

            .subcommand(SubCommand::with_name("mark-review-as-read")
                .about("Mark review as read.")
                .arg(Arg::with_name("review-update-url")
                    .help("URL to the review update API.")
                    .long("review-update-url")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("get-unread-reviews")
                .about("Get unread reviews.")
                .arg(Arg::with_name("reviews-url")
                    .help("URL to the reviews API.")
                    .long("reviews-url")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("request-code-review")
                .about("Request code review.")
                .arg(Arg::with_name("submission-url")
                    .help("URL where the submission should be posted.")
                    .long("submission-url")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("submission-path")
                    .help("Path to the directory where the submission resides.")
                    .long("submission-path")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("message-for-reviewer")
                    .help("Message for the review.")
                    .long("message-for-reviewer")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("locale")
                    .help("Language as a three letter ISO 639-3 code, e.g. 'eng' or 'fin'.")
                    .long("locale")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("download-model-solution")
                .about("Download model solutions.")
                .arg(Arg::with_name("solution-download-url")
                    .help("URL to the solution download.")
                    .long("solution-download-url")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("target")
                    .help("Path to where the model solution will be downloaded.")
                    .long("target")
                    .required(true)
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("reset-exercise")
                .about("Reset exercise, optionally submitting it before doing so.")
                .arg(Arg::with_name("exercise-path")
                    .help("Path to the directory where the project resides.")
                    .long("exercise-path")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("save-old-state")
                    .help("If set, the exercise is submitted to the server before resetting it.")
                    .long("save-old-state")
                    .requires("submission-url"))
                .arg(Arg::with_name("submission-url")
                    .help("Required if save-old-state is set. The URL where the submission should be posted.")
                    .long("submission-url")
                    .takes_value(true)))

            .subcommand(SubCommand::with_name("download-old-submission")
                .about("Downloads an old submission.")
                .arg(Arg::with_name("exercise-id")
                    .help("The ID of the exercise.")
                    .long("exercise-id")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("submission-id")
                    .help("The ID of the submission.")
                    .long("submission-id")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("output-path")
                    .help("Path to where the submission should be downloaded.")
                    .long("output-path")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("save-old-state")
                    .help("If set, the exercise is submitted to the server before resetting it.")
                    .long("save-old-state")
                    .requires("submission-url"))
                .arg(Arg::with_name("submission-url")
                    .help("Required if save-old-state is set. The URL where the submission should be posted.")
                    .long("submission-url")
                    .takes_value(true))))
}
