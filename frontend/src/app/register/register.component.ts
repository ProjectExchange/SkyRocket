import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthUser, UsersService } from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.sass'],
})
export class RegisterComponent {
  registerForm: FormGroup;

  constructor(
    private authService: AuthService,
    private formBuilder: FormBuilder,
    private router: Router,
    private usersService: UsersService,
  ) {
    this.registerForm = this.formBuilder.group({
      firstname: [this.authService.firstname, [Validators.required.bind(this)]],
      lastname: [this.authService.lastname, [Validators.required.bind(this)]],
      email: [
        this.authService.email,
        [Validators.required.bind(this), Validators.email.bind(this)],
      ],
    });
  }

  form(control: string): string {
    return this.registerForm.controls[control].value;
  }

  onSubmit(): void {
    if (!this.registerForm.valid) return;
    this.usersService
      .create({
        email: this.form('email'),
        firstname: this.form('firstname'),
        lastname: this.form('lastname'),
      })
      .subscribe((user: AuthUser) => {
        this.authService.user = user;
        this.router.navigate(['/profile']);
      });
  }
}
