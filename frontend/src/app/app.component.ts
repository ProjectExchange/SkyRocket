import { Component } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { LoginService } from '@skyrocket/ng-api-client';
import { UserService } from './_services/user.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass']
})
export class AppComponent {
  title = 'SkyRocket';

  constructor(
    private titleService: Title,
    public userService: UserService,
  ) {
    this.titleService.setTitle('SkyRocket');
  }
}
