use crate::models::{
    permissions::Permission,
    user::UserRole,
};

pub fn permissions_for_role(
    role: &UserRole,
) -> Vec<Permission> {
    match role {
        UserRole::SuperAdmin => {
            vec![
                Permission::UsersRead,
                Permission::UsersCreate,
                Permission::UsersUpdate,
                Permission::UsersDelete,

                Permission::StudentsRead,
                Permission::StudentsCreate,
                Permission::StudentsUpdate,
                Permission::StudentsDelete,

                Permission::LecturersRead,
                Permission::LecturersCreate,
                Permission::LecturersUpdate,
                Permission::LecturersDelete,

                Permission::CoursesRead,
                Permission::CoursesCreate,
                Permission::CoursesUpdate,
                Permission::CoursesDelete,

                Permission::ResultsRead,
                Permission::ResultsEnter,
                Permission::ResultsUpdate,
                Permission::ResultsApprove,
            ]
        }

        _ => {
            vec![]
        }
    }
}

pub fn has_permission(
    role: &UserRole,
    permission: &Permission,
) -> bool {
    permissions_for_role(role)
        .contains(permission)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::UserRole;

    #[test]
    fn super_admin_has_users_create_permission() {
        assert!(
            has_permission(
                &UserRole::SuperAdmin,
                &Permission::UsersCreate,
            )
        );
    }

    #[test]
    fn super_admin_has_results_approve_permission() {
        assert!(
            has_permission(
                &UserRole::SuperAdmin,
                &Permission::ResultsApprove,
            )
        );
    }
}