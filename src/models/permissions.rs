use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Permission {
    UsersRead,
    UsersCreate,
    UsersUpdate,
    UsersDelete,

    StudentsRead,
    StudentsCreate,
    StudentsUpdate,
    StudentsDelete,

    LecturersRead,
    LecturersCreate,
    LecturersUpdate,
    LecturersDelete,

    CoursesRead,
    CoursesCreate,
    CoursesUpdate,
    CoursesDelete,

    ResultsRead,
    ResultsEnter,
    ResultsUpdate,
    ResultsApprove,
}