import { Injectable } from '@angular/core';
import { AuthUser, NewUser } from '@skyrocket/ng-api-client';

function isAuthUser(object: NewUser | AuthUser): object is AuthUser {
  return 'id' in object;
}

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  #user?: NewUser | AuthUser;

  constructor() { /**/ }

  set user(user: NewUser | AuthUser | undefined) {
    this.#user = user;
  }

  get firstname(): string {
    return this.#user?.firstname || '';
  }

  get lastname(): string {
    return this.#user?.lastname || '';
  }

  get email(): string {
    return this.#user?.email || '';
  }

  get isLoggedIn(): boolean {
    if (!this.#user) return false;
    return isAuthUser(this.#user);
  }
}
