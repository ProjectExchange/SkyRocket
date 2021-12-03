mod github_oauth_user;
mod role;
mod user;

pub use github_oauth_user::{GitHubOAuthUser, GithubOAuthRegistrar};
pub use role::UserRole;
pub use user::{AuthUser, NewUser, User};
pub use role::RoleMapping;

pub fn register_polar_classes(oso: &mut oso::Oso) -> oso::Result<()> {
    user::register_polar_classes(oso)
}