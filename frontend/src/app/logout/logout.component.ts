import { Component, ElementRef, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { LoginService, UsersService } from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';

@Component({
  selector: 'app-logout',
  templateUrl: './logout.component.html',
  styleUrls: ['./logout.component.sass'],
})
export class LogoutComponent implements OnInit {
  constructor(
    private authService: AuthService,
    private elementRef: ElementRef,
    private loginService: LoginService,
    private router: Router,
  ) {}

  ngOnInit(): void {}

  ngAfterViewInit() {
    this.elementRef.nativeElement.ownerDocument.body.style.backgroundImage = 'url("../../assets/img/berg-5128982_1920.jpg")';
  }

  logout() {
    this.loginService.logout().subscribe();
    this.authService.user = undefined;
    this.router.navigate(['/']);
  }
}
