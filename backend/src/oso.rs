use crate::db::models::AuthUser;
use oso::{Oso, Result, ToPolar};
use std::fmt;
use std::sync::{Arc, Mutex};

pub struct OsoArc {
    oso: Arc<Mutex<Oso>>,
}

pub enum OsoAction {
    Create,
    Read,
    Update,
    Delete,
}

impl fmt::Display for OsoAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OsoAction::Create => write!(f, "create"),
            OsoAction::Read => write!(f, "read"),
            OsoAction::Update => write!(f, "update"),
            OsoAction::Delete => write!(f, "delete"),
        }
    }
}

impl OsoArc {
    pub fn is_allowed<Resource>(
        &self,
        actor: AuthUser,
        action: OsoAction,
        resource: Resource,
    ) -> bool
    where
        Resource: ToPolar,
    {
        let guard = self.oso.lock().unwrap();
        guard
            .is_allowed(actor, action.to_string(), resource)
            .unwrap()
    }
}

pub type OsoState = rocket::State<OsoArc>;

fn init_oso_arc() -> Result<OsoArc> {
    let mut oso = Oso::new();

    crate::db::models::register_polar_classes(&mut oso)?;

    oso.load_files(vec!["security/users.polar", "security/addresses.polar"])?;

    Ok(OsoArc {
        oso: Arc::new(Mutex::new(oso)),
    })
}

pub fn init() -> OsoArc {
    init_oso_arc().unwrap_or_else(|e| {
        eprintln! { "Error loading oso policies:\n{}", e };
        std::process::exit(1);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::{Address, AuthUser, User};
    use once_cell::sync::Lazy;

    static OSO: Lazy<OsoArc> = Lazy::new(init);

    #[test]
    fn test_user_read_self() {
        assert_eq!(
            true,
            OSO.is_allowed(AuthUser::dummy(1), OsoAction::Read, User::dummy(1))
        );
    }

    #[test]
    fn test_user_read_other_user() {
        assert_eq!(
            false,
            OSO.is_allowed(AuthUser::dummy(1), OsoAction::Read, User::dummy(2))
        );
    }

    #[test]
    fn test_admin_user_read_self() {
        assert_eq!(
            true,
            OSO.is_allowed(AuthUser::dummy_admin(1), OsoAction::Read, User::dummy(1))
        );
    }

    #[test]
    fn test_admin_user_read_other_user() {
        assert_eq!(
            true,
            OSO.is_allowed(AuthUser::dummy_admin(1), OsoAction::Read, User::dummy(2))
        );
    }

    #[test]
    fn test_user_read_own_addresses() {
        assert_eq!(
            true,
            OSO.is_allowed(AuthUser::dummy(1), OsoAction::Read, Address::dummy_for_user(1))
        );
    }

    #[test]
    fn test_user_read_other_addresses() {
        assert_eq!(
            false,
            OSO.is_allowed(AuthUser::dummy(1), OsoAction::Read, Address::dummy_for_user(2))
        );
    }

    #[test]
    fn test_admin_user_read_own_addresses() {
        assert_eq!(
            true,
            OSO.is_allowed(AuthUser::dummy_admin(1), OsoAction::Read, Address::dummy_for_user(1))
        );
    }

    #[test]
    fn test_admin_user_read_other_addresses() {
        assert_eq!(
            true,
            OSO.is_allowed(AuthUser::dummy_admin(1), OsoAction::Read, Address::dummy_for_user(2))
        );
    }
}
