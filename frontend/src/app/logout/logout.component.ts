import { Component, ElementRef } from '@angular/core';
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
    private elementRef: ElementRef,
    private loginService: LoginService,
    private router: Router,
  ) {}

  ngAfterViewInit() {
    this.elementRef.nativeElement.ownerDocument.body.style.backgroundImage = 'url("../../assets/img/berg-5128982_1920.jpg")';
  }

  logout() {
    this.loginService.logout().subscribe();
    this.authService.user = undefined;
    this.router.navigate(['/']);
  }
}
