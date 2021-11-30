import { Injectable } from '@angular/core';
import { User } from '@skyrocket/ng-api-client';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  #user?: User;

  constructor() { }

  set user(user: User) {
    this.#user = user;
  }

  get firstname(): string {
    return this.#user?.firstname || '';
  }

  get lastname(): string {
    return this.#user?.lastname || '';
  }

  get isLoggedIn(): boolean {
    return !!this.#user?.id;
  }
}
