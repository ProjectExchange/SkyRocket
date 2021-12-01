export * from './login.service';
import { LoginService } from './login.service';
export * from './users.service';
import { UsersService } from './users.service';
export const APIS = [LoginService, UsersService];
