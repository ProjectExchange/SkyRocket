import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { LoginService } from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';

@Component({
  selector: 'app-logout',
  templateUrl: './logout.component.html',
  styleUrls: ['./logout.component.sass'],
})
export class LogoutComponent {
  constructor(
    private authService: AuthService,
    private loginService: LoginService,
    private router: Router,
  ) {}

  logout() {
    this.loginService.logout().subscribe();
    this.authService.user = undefined;
    this.router.navigate(['/']);
  }
}
