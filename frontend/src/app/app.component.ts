import { Component } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { AuthService } from './_services/auth.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass'],
})
export class AppComponent {
  title = 'SkyRocket';

  constructor(
    private titleService: Title,
    public authService: AuthService,
  ) {
    this.titleService.setTitle('SkyRocket');
  }
}
