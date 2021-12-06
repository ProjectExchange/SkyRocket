import { User, UsersService } from '@skyrocket/ng-api-client';
import { AuthService } from './auth.service';

export function appInitializer(
  authService: AuthService,
  userService: UsersService,
) {
  return (): Promise<void> => new Promise((resolve) => {
    userService
      .profile()
      .subscribe((user: User) => {
        /* eslint no-param-reassign: "error" */
        authService.user = user;
      })
      .add(resolve);
  });
}
