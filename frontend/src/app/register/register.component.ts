import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthUser, Gender, UsersService } from '@skyrocket/ng-api-client';
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
      birthday: ['', [Validators.required.bind(this)]],
      gender: ['', [Validators.required.bind(this)]],
    });
  }

  form(name: string): string {
    return this.registerForm.controls[name].value;
  }

  isInvalid(name: string): boolean {
    const control = this.registerForm.controls[name];
    return control.touched && control.invalid;
  }

  onSubmit(): void {
    if (!this.registerForm.valid) return;
    this.usersService
      .create({
        email: this.form('email'),
        firstname: this.form('firstname'),
        lastname: this.form('lastname'),
        birthday: new Date(this.form('birthday'))
          .toISOString()
          .split('T')[0]
          .toString(),
        gender: this.form('gender') as Gender,
      })
      .subscribe((user: AuthUser) => {
        this.authService.user = user;
        this.router.navigate(['/profile']);
      });
  }
}
