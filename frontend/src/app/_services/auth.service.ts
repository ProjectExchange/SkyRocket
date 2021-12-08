import { Injectable } from '@angular/core';
import { AuthUser, NewUser } from '@skyrocket/ng-api-client';

function isAuthUser(object: NewUser | AuthUser | undefined): object is AuthUser {
  return !!object && 'id' in object;
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

  get id(): number {
    return isAuthUser(this.#user) ? this.#user.id : 0;
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
    return isAuthUser(this.#user);
  }
}
