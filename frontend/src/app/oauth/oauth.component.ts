import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { LoginService } from '@skyrocket/ng-api-client';
import { UserService } from '../_services/user.service';

@Component({
  selector: 'app-oauth',
  templateUrl: './oauth.component.html',
  styleUrls: ['./oauth.component.sass']
})
export class OauthComponent implements OnInit {

  constructor(
    private activatedRoute: ActivatedRoute,
    private loginService: LoginService,
    private userService: UserService,
    private router: Router,
  ) { }

  ngOnInit(): void {
    const service = this.activatedRoute.snapshot.paramMap.get('service');

    switch (service?.toLowerCase()) {
      case 'github':
        this.activatedRoute.queryParams.subscribe((params) => {
          const code = params['code'];
          this.loginService.loginGithub(code).subscribe((user) => {
            this.userService.user = user;
            // valid user if id is already set
            if (this.userService.isLoggedIn) {
              this.router.navigate(['/profile']);
            } else {
              this.router.navigate(['/register']);
            }
          })
        });
        break;
      default:
        break;
    }
  }
}
