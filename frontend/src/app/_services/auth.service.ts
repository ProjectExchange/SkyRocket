import { Injectable } from '@angular/core';
import { AuthUser, NewUser } from '@skyrocket/ng-api-client';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  #user?: NewUser | AuthUser;

  constructor() { /**/ }

  isAuthUser(object: NewUser | AuthUser): object is AuthUser {
    return 'id' in object;
  }

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
    return this.isAuthUser(this.#user);
  }
}
