actor AuthUser {}

resource User {
  permissions = ["read", "push"];
  roles = ["self", "Admin"];

  "read" if "Admin";
  "read" if "self";
}

has_role(actor: AuthUser, "self", resource: User) if
  actor.id = resource.id;

has_role(actor: AuthUser, name: String, _: User) if
  role in actor.roles and role = name;

allow(actor, action, resource) if
  has_permission(actor, action, resource);

