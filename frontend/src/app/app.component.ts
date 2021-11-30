import { Component } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { UserService } from './_services/user.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass'],
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
