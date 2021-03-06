//! Ant student file policy

use std::path::Path;
use tmc_langs_framework::{policy::StudentFilePolicy, TmcProjectYml};

pub struct AntStudentFilePolicy {
    project_config: TmcProjectYml,
}

impl StudentFilePolicy for AntStudentFilePolicy {
    fn new_with_project_config(project_config: TmcProjectYml) -> Self
    where
        Self: Sized,
    {
        Self { project_config }
    }

    fn get_project_config(&self) -> &TmcProjectYml {
        &self.project_config
    }

    fn is_student_source_file(path: &Path) -> bool {
        path.starts_with("src")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_student_source_file() {
        assert!(AntStudentFilePolicy::is_student_source_file(Path::new(
            "src/file"
        )));
        assert!(AntStudentFilePolicy::is_student_source_file(Path::new(
            "src/dir/file"
        )));
    }

    #[test]
    fn is_not_student_source_file() {
        assert!(!AntStudentFilePolicy::is_student_source_file(Path::new(
            "file"
        )));
        assert!(!AntStudentFilePolicy::is_student_source_file(Path::new(
            "dir/src/file"
        )));
        assert!(!AntStudentFilePolicy::is_student_source_file(Path::new(
            "srca/file"
        )));
    }
}
