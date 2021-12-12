import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import {
  Address, AddressesService, FlightOfferWithOccupancy, FlightsService, Session, SessionsService,
} from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';

interface BookingPrint {
  departure: string,
  arrival: string,
  seats: number,
}

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

  bookings: BookingPrint[] = [];

  bookingDisplayedColumns: string[] = [
    'departure',
    'arrival',
    'seats',
  ];

  offers: {[key: number]: FlightOfferWithOccupancy} = {};

  sessions: Session[] = [];

  sessionDisplayedColumns: string[] = [
    'established',
    'device',
    'actions',
  ];

  constructor(
    private addressForm: FormBuilder,
    private addressesService: AddressesService,
    private authService: AuthService,
    private sessionService: SessionsService,
    private flightService: FlightsService,
  ) {
    this.profileAddressForm = this.addressForm.group({
      street: ['', [Validators.required.bind(this)]],
      houseNumber: ['', [Validators.required.bind(this)]],
      postalCode: ['', [Validators.required.bind(this)]],
      town: ['', [Validators.required.bind(this)]],
      country: ['', [Validators.required.bind(this)]],
    });
  }

  formatDate(date: string): string {
    return date.split('T').slice(1, 4).join(' ');
  }

  ngOnInit() {
    this.addressesService.read(this.authService.id).subscribe((addresses) => {
      this.dataSource = addresses;
    });
    this.updateSessionsTable();

    this.flightService.readOffer().subscribe((offers) => {
      offers.forEach((offer) => {
        this.offers[offer.id] = offer;
      });
      this.updateBookingsTable();
    });
  }

  updateBookingsTable() {
    this.flightService.readOfferBookings(this.authService.id).subscribe((bookings) => {
      this.bookings = bookings.map((booking) => {
        if (this.offers[booking.offerId]) {
          return {
            departure: this.offers[booking.offerId].departureIcao,
            arrival: this.offers[booking.offerId].arrivalIcao,
            seats: booking.seats,
          };
        }
        return {
          departure: 'n/a',
          arrival: 'n/a',
          seats: booking.seats,
        };
      });
    });
  }

  updateSessionsTable() {
    this.sessionService.read(this.authService.id).subscribe((sessions) => {
      this.sessions = sessions;
    });
  }

  form(name: string): string {
    return this.profileAddressForm.controls[name].value;
  }

  isInvalid(name: string): boolean {
    const control = this.profileAddressForm.controls[name];
    return control.touched && control.invalid;
  }

  revokeSession(id: number) {
    // eslint-disable-next-line no-underscore-dangle
    this.sessionService._delete(this.authService.id, id).subscribe();
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
