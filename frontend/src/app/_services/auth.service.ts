import { Injectable } from '@angular/core';
import { User } from '@skyrocket/ng-api-client';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  #user?: User;

  constructor() { /**/ }

  set user(user: User | undefined) {
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
    return !!this.#user?.id;
  }
}
