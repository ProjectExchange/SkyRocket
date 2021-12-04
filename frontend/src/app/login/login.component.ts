import { Component, ElementRef, OnInit } from '@angular/core';
import { LoginService, OAuthProviders } from '@skyrocket/ng-api-client';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.sass'],
})
export class LoginComponent implements OnInit {
  public oAuthEndpoints: OAuthProviders = {};

  constructor(
    private loginService: LoginService,
    private elementRef: ElementRef,
  ) {}

  ngOnInit(): void {
    this.fetchOAuthEndpoints();
  }

  ngAfterViewInit() {
    this.elementRef.nativeElement.ownerDocument.body.style.backgroundImage = 'url("../../assets/img/berg-5128982_1920.jpg")';
  }

  fetchOAuthEndpoints(): void {
    this.loginService.oauthList().subscribe((data: OAuthProviders) => {
      this.oAuthEndpoints = data;
    });
  }
}
