actor AuthUser {}

resource Booking {
  permissions = ["read"];
  roles = ["self", "Admin"];

  # admin user has all rights, that a user has on his own account
  "self" if "Admin";

  "read" if "self";
}

has_role(actor: AuthUser, "self", resource: Booking) if
  actor.id = resource.user_id;

has_role(actor: AuthUser, name: String, _: Booking) if
  role in actor.roles and role = name;

allow(actor, action, resource) if
  has_permission(actor, action, resource);
