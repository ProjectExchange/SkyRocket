actor AuthUser {}

resource Address {
  permissions = ["create", "read", "update", "delete"];
  roles = ["self", "Admin"];

  # admin user has all rights, that a user has on his own account
  "self" if "Admin";

  "create" if "self";
  "read" if "self";
  "update" if "self";
  "delete" if "self";
}

has_role(actor: AuthUser, "self", resource: Address) if
  actor.id = resource.user_id;

has_role(actor: AuthUser, name: String, _: Address) if
  role in actor.roles and role = name;

allow(actor, action, resource) if
  has_permission(actor, action, resource);
