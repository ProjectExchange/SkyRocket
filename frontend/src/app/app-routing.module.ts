import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { BookComponent } from './book/book.component';
import { HomeComponent } from './home/home.component';
import { LoginComponent } from './login/login.component';
import { LogoutComponent } from './logout/logout.component';
import { ManagementComponent } from './management/management.component';
import { OauthComponent } from './oauth/oauth.component';
import { ProfileComponent } from './profile/profile.component';
import { RegisterComponent } from './register/register.component';
import { AuthenticatedGuard } from './_guards/authenticated.guard';
import { LoggedOutGuard } from './_guards/logged-out.guard';

const routes: Routes = [
  { path: 'book', component: BookComponent, canActivate: [AuthenticatedGuard] },
  {
    path: 'login',
    canActivate: [LoggedOutGuard],
    children: [
      { path: 'oauth/:service', component: OauthComponent },
      { path: '', component: LoginComponent },
    ],
  },
  { path: 'logout', component: LogoutComponent, canActivate: [AuthenticatedGuard] },
  { path: 'management', component: ManagementComponent, canActivate: [AuthenticatedGuard] },
  { path: 'profile', component: ProfileComponent, canActivate: [AuthenticatedGuard] },
  { path: 'register', component: RegisterComponent, canActivate: [LoggedOutGuard] },
  { path: '', component: HomeComponent },
  { path: '**', redirectTo: '' },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule { }
