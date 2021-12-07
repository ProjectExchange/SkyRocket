mod address;
mod flight;
mod github_oauth_user;
mod role;
mod user;

pub use address::{Address, NewAddress};
pub use flight::{Currency, CurrencyMapping, Flight, FlightOffer, NewFlight, NewFlightOffer};
pub use github_oauth_user::{GitHubOAuthUser, GithubOAuthRegistrar};
pub use role::{AdminRole, Role, RoleMapping, UserRole};
pub use user::{AuthUser, Gender, GenderMapping, NewUser, User};

pub(self) type DbResult = Result<usize, diesel::result::Error>;

pub fn register_polar_classes(oso: &mut oso::Oso) -> oso::Result<()> {
    user::register_polar_classes(oso)?;
    address::register_polar_classes(oso)
}
