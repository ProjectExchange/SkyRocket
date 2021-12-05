mod github_oauth_user;
mod role;
mod user;

pub use github_oauth_user::{GitHubOAuthUser, GithubOAuthRegistrar};
pub use role::{Role, RoleMapping, UserRole, AdminRole};
pub use user::{AuthUser, NewUser, User};

pub(self) type DbResult = Result<usize, diesel::result::Error>;

pub fn register_polar_classes(oso: &mut oso::Oso) -> oso::Result<()> {
    user::register_polar_classes(oso)
}
