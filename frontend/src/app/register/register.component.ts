import { Component, ElementRef, NgZone, OnInit } from '@angular/core';
import {
  FormBuilder, FormGroup, Validators,
} from '@angular/forms';
import { User, UsersService } from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.sass'],
})
export class RegisterComponent implements OnInit {
  registerForm: FormGroup;

  constructor(
    private authService: AuthService,
    private elementRef: ElementRef,
    private formBuilder: FormBuilder,
    private usersService: UsersService,
  ) {
    this.registerForm = this.formBuilder.group({
      firstname: [
        this.authService.firstname,
        [Validators.required.bind(this)],
      ],
      lastname: [
        this.authService.lastname,
        [Validators.required.bind(this)],
      ],
      email: [
        this.authService.email,
        [Validators.required.bind(this), Validators.email.bind(this)],
      ],
    });
  }

  ngOnInit(): void {}

  ngAfterViewInit() {
    this.elementRef.nativeElement.ownerDocument.body.style.backgroundImage = 'url("../../assets/img/berg-5128982_1920.jpg")';
  }

  form(control: string): string {
    return this.registerForm.controls[control].value;
  }

  register(): Promise<unknown> {
    return new Promise(() => {
      this.usersService.create({
        email: this.form('email'),
        firstname: this.form('firstname'),
        lastname: this.form('lastname'),
      }).subscribe((user: User) => {
        alert(user);
        this.authService.user = {
          ...user, id: 1
        };
      });
    });
  }

  onSubmit(): void {
    if (!this.registerForm.valid) return;
    this.register().then(() => {
      alert(this.authService.user);
    });
  }
}
