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
