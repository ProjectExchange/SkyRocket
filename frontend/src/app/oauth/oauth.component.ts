import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { LoginService, User } from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';

@Component({
  selector: 'app-oauth',
  templateUrl: './oauth.component.html',
  styleUrls: ['./oauth.component.sass'],
})
export class OauthComponent implements OnInit {
  constructor(
    private activatedRoute: ActivatedRoute,
    private loginService: LoginService,
    private authService: AuthService,
    private router: Router,
  ) { }

  ngOnInit(): void {
    const service = this.activatedRoute.snapshot.paramMap.get('service');

    switch (service?.toLowerCase()) {
      case 'github':
        this.activatedRoute.queryParams.subscribe((params) => {
          const { code } = params;
          this.loginService.loginGithub(code).subscribe((user: User) => {
            this.authService.user = user;
            // valid user if id is already set
            if (this.authService.isLoggedIn) {
              this.router.navigate(['/profile']);
            } else {
              this.router.navigate(['/register']);
            }
          });
        });
        break;
      default:
        break;
    }
  }
}
