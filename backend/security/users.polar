actor AuthUser {}

resource User {
  permissions = ["read", "update", "delete"];
  roles = ["self", "Admin"];

  # admin user has all rights, that a user has on his own account
  "self" if "Admin";

  "read" if "self";
  "update" if "self";
  "delete" if "self";
}

has_role(actor: AuthUser, "self", resource: User) if
  actor.id = resource.id;

has_role(actor: AuthUser, name: String, _: User) if
  role in actor.roles and role = name;

allow(actor, action, resource) if
  has_permission(actor, action, resource);

