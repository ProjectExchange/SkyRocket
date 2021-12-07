export * from './addresses.service';
import { AddressesService } from './addresses.service';
export * from './flights.service';
import { FlightsService } from './flights.service';
export * from './login.service';
import { LoginService } from './login.service';
export * from './users.service';
import { UsersService } from './users.service';
export const APIS = [AddressesService, FlightsService, LoginService, UsersService];
