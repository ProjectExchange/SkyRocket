import { Component, OnInit } from '@angular/core';
import { LoginService, OAuthProviders } from '@skyrocket/ng-api-client';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.sass']
})
export class LoginComponent implements OnInit {

  public oAuthEndpoints: OAuthProviders = {};

  constructor(
    private loginService: LoginService
  ) {}

  ngOnInit(): void {
    this.fetchOAuthEndpoints();
  }

  fetchOAuthEndpoints(): void {
    this.loginService.oauthList().subscribe((data) => {
      this.oAuthEndpoints = data;
    })
  }
}
