import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import {
  Currency,
  Flight,
  FlightOffer,
  FlightsService,
} from '@skyrocket/ng-api-client';

interface FlightOfferPrint {
  value: number;
  viewValue: string;
}

@Component({
  selector: 'app-management',
  templateUrl: './management.component.html',
  styleUrls: ['./management.component.sass'],
})
export class ManagementComponent implements OnInit {
  managementFlightForm: FormGroup;

  managementFlightOfferForm: FormGroup;

  displayedColumnsFlight: string[] = [
    'offerId',
    'departureIcao',
    'departureTime',
    'arrivalIcao',
    'arrivalTime',
  ];

  displayedColumnsFlightOffer: string[] = ['id', 'seats', 'price', 'currency'];

  dataSourceFlight: Flight[] = [];

  dataSourceFlightOffer: FlightOffer[] = [];

  flightOffers: FlightOfferPrint[] = [];

  currencies: string[] = Object.keys(Currency);

  constructor(
    private flightForm: FormBuilder,
    private flightOfferForm: FormBuilder,
    private flightService: FlightsService,
  ) {
    this.managementFlightForm = this.flightForm.group({
      idFlightOffer: ['', [Validators.required.bind(this)]],
      departureIcao: ['', [Validators.required.bind(this)]],
      departureTime: ['', [Validators.required.bind(this)]],
      arrivalIcao: ['', [Validators.required.bind(this)]],
      arrivalTime: ['', [Validators.required.bind(this)]],
    });

    this.managementFlightOfferForm = this.flightOfferForm.group({
      seats: [
        '',
        [
          Validators.required.bind(this),
          Validators.min(0),
          Validators.max(1000),
        ],
      ],
      price: [
        '',
        [
          Validators.required.bind(this),
          Validators.min(0),
          Validators.max(10000),
        ],
      ],
      currency: ['', [Validators.required.bind(this)]],
    });
  }

  form(name: string, formGroup: FormGroup): string {
    return formGroup.controls[name].value;
  }

  isInvalid(name: string): boolean {
    const control = this.managementFlightForm.controls[name];
    return control.touched && control.invalid;
  }

  ngOnInit(): void {
    // clear due push below
    this.flightOffers = [];

    this.flightService.readOffer().subscribe((offers) => {
      this.dataSourceFlightOffer = offers;
      this.dataSourceFlightOffer.forEach((flightOffer) => {
        this.flightOffers.push({
          value: flightOffer.id,
          viewValue: ''.concat(
            'ID ',
            flightOffer.id.toString(),
            ', ',
            flightOffer.seats.toString(),
            ' seats, ',
            flightOffer.price.toString(),
            ' ',
            flightOffer.currency,
          ),
        });
        this.flightService.readFlights(flightOffer.id).subscribe((flights) => {
          this.dataSourceFlight = [...this.dataSourceFlight, ...flights];
        });
      });
    });
  }

  onSubmitFlight() {
    if (!this.managementFlightForm.valid) return;
    this.flightService
      .createFlights(
        [
          {
            departureIcao: this.form(
              'departureIcao',
              this.managementFlightForm,
            ),
            departureTime: new Date(
              this.form('departureTime', this.managementFlightForm),
            ),
            arrivalIcao: this.form('arrivalIcao', this.managementFlightForm),
            arrivalTime: new Date(
              this.form('arrivalTime', this.managementFlightForm),
            ),
          },
        ],
        parseInt(this.form('idFlightOffer', this.managementFlightForm), 10),
      )
      .subscribe(() => {
        this.ngOnInit();
      });
  }

  onSubmitFlightOffer() {
    if (!this.managementFlightOfferForm.valid) return;
    this.flightService
      .createOffer({
        seats: parseInt(this.form('seats', this.managementFlightOfferForm), 10),
        price: parseInt(this.form('price', this.managementFlightOfferForm), 10),
        currency: this.form(
          'currency',
          this.managementFlightOfferForm,
        ) as Currency,
      })
      .subscribe(() => {
        this.ngOnInit();
      });
  }

  formatDate(date: string): string {
    return new Date(date).toISOString().split('.')[0].replace('T', ' ');
  }
}
