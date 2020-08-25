initSidebarItems({"enum":[["OutputFormat",""]],"fn":[["clean","See `LanguagePlugin::clean`."],["compress_project","See `LanguagePlugin::compress_project`."],["compress_tar_for_submitting","Creates a tarball that can be submitted to TMC-sandbox."],["extract_project","Finds the correct language plug-in for the given exercise path and calls `LanguagePlugin::extract_project`, If no language plugin matches, see `extract_project_overwrite`."],["extract_project_overwrite","Extract a given archive file containing a compressed project to a target location. This will overwrite any existing files."],["extract_student_files","Extracts a project archive, only taking out files classified as student files."],["find_exercise_directories","Recursively searches for valid exercise directories in the path."],["get_exercise_packaging_configuration","See `LanguagePlugin::get_exercise_packaging_configuration`."],["is_exercise_root_directory","Tries to find a language plugin for the path, returning `true` if one is found."],["prepare_solutions","See `domain::prepare_solutions`."],["prepare_stubs","See `LanguagePlugin::prepare_stubs`."],["prepare_submission","Takes a submission zip and turns it into an archive suitable for further processing by among other things resetting the test files"],["run_check_code_style","See `LanguagePlugin::check_code_style`."],["run_tests","See `LanguagePlugin::run_tests`."],["scan_exercise","See `LanguagePlugin::scan_exercise`."]],"struct":[["TmcParams","TmcParams is used to safely construct data for a .tmcparams file, which contains lines in the form of export A=B export C=(D, E, F) the keys and values of the inner hashmap are validated to make sure they are valid as bash variables"]]});