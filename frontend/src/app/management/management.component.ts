import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import {
  Currency,
  Flight,
  FlightOffer,
  FlightOfferWithOccupancy,
  FlightsService,
  UsersService,
} from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';

interface BookingPrint {
  offerId: number,
  departure: string,
  arrival: string,
  seats: number,
}

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

  bookings: BookingPrint[] = [];

  bookingDisplayedColumns: string[] = [
    'offerId',
    'departure',
    'arrival',
    'seats',
  ];

  offers: {[key: number]: FlightOfferWithOccupancy} = {};

  currencies: string[] = Object.keys(Currency);

  constructor(
    private authService: AuthService,
    private flightForm: FormBuilder,
    private flightOfferForm: FormBuilder,
    private flightService: FlightsService,
    private usersService: UsersService,
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
          Validators.min(1),
          Validators.max(2000),
        ],
      ],
      price: [
        '',
        [
          Validators.required.bind(this),
          Validators.min(1),
          Validators.max(99999),
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
    this.dataSourceFlight = [];

    this.flightService.readOfferRaw().subscribe((offers) => {
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

    this.flightService.readOffer().subscribe((offers) => {
      offers.forEach((offer) => {
        this.offers[offer.id] = offer;
      });
      this.updateBookingsTable();
    });
  }

  updateBookingsTable() {
    this.usersService.listForUser().subscribe((users) => {
      users.forEach((user) => {
        this.flightService.readOfferBookings(user.id).subscribe((bookings) => {
          this.bookings = [...this.bookings, ...bookings.map((booking) => {
            if (this.offers[booking.offerId]) {
              return {
                offerId: booking.offerId,
                departure: this.offers[booking.offerId].departureIcao,
                arrival: this.offers[booking.offerId].arrivalIcao,
                seats: booking.seats,
              };
            }
            return {
              offerId: booking.offerId,
              departure: 'n/a',
              arrival: 'n/a',
              seats: booking.seats,
            };
          })];
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
