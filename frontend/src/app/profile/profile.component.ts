import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Address, AddressesService } from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';

@Component({
  selector: 'app-profile',
  templateUrl: './profile.component.html',
  styleUrls: ['./profile.component.sass'],
})
export class ProfileComponent implements OnInit {
  profileAddressForm: FormGroup;

  displayedColumns: string[] = [
    'street',
    'houseNumber',
    'postalCode',
    'town',
    'country',
  ];

  dataSource: Address[] = [];

  constructor(
    private addressForm: FormBuilder,
    private addressesService: AddressesService,
    private authService: AuthService,
  ) {
    this.profileAddressForm = this.addressForm.group({
      street: ['', [Validators.required.bind(this)]],
      houseNumber: ['', [Validators.required.bind(this)]],
      postalCode: ['', [Validators.required.bind(this)]],
      town: ['', [Validators.required.bind(this)]],
      country: ['', [Validators.required.bind(this)]],
    });
  }

  ngOnInit() {
    this.addressesService.read(this.authService.id).subscribe((addresses) => {
      this.dataSource = addresses;
    });
  }

  form(name: string): string {
    return this.profileAddressForm.controls[name].value;
  }

  isInvalid(name: string): boolean {
    const control = this.profileAddressForm.controls[name];
    return control.touched && control.invalid;
  }

  onSubmit() {
    if (!this.profileAddressForm.valid) return;
    this.addressesService
      .create(
        {
          country: JSON.parse(JSON.stringify(this.form('country'))).name,
          postalCode: parseInt(this.form('postalCode'), 10),
          town: this.form('town'),
          street: this.form('street'),
          houseNumber: parseInt(this.form('houseNumber'), 10),
        },
        this.authService.id,
      )
      .subscribe(() => {
        this.ngOnInit();
      });
  }
}
