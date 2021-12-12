import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import {
  Address,
  AddressesService,
  FlightOfferWithOccupancy,
  FlightsService,
} from '@skyrocket/ng-api-client';
import { AuthService } from '../_services/auth.service';
import { BookingService } from '../_services/booking.service';

@Component({
  selector: 'app-book',
  templateUrl: './book.component.html',
  styleUrls: ['./book.component.sass'],
})
export class BookComponent implements OnInit {
  bookFormStep1: FormGroup;

  bookFormStep2: FormGroup;

  bookFormStep3: FormGroup;

  offers: FlightOfferWithOccupancy[] = [];

  addresses: Address[] = [];

  constructor(
    private authService: AuthService,
    private addressService: AddressesService,
    private bookForm: FormBuilder,
    private flightService: FlightsService,
    private router: Router,
    public bookingService: BookingService,
  ) {
    this.bookFormStep1 = this.bookForm.group({
      flight: ['', Validators.required],
      seats: [
        '',
        [
          Validators.required.bind(this),
          Validators.min(1),
          Validators.max(2000),
        ],
      ],
    });

    this.bookFormStep2 = this.bookForm.group({
      firstname: [this.authService.firstname, [Validators.required.bind(this)]],
      lastname: [this.authService.lastname, [Validators.required.bind(this)]],
    });

    this.bookFormStep3 = this.bookForm.group({
      address: ['', Validators.required],
    });
  }

  formatDate(date: Date): string {
    return date.toString()
      .split(' ')
      .slice(1, 4)
      .join(' ');
  }

  get dateDeparture(): string {
    return this.formatDate(this.bookingService.dateDeparture);
  }

  get dateArrival(): string {
    return this.formatDate(this.bookingService.dateArrival);
  }

  form(name: string, formGroup: FormGroup): string {
    return formGroup.controls[name].value;
  }

  bookNow(): void {
    this.flightService.createOfferBooking(
      parseInt(this.form('flight', this.bookFormStep1), 10),
      parseInt(this.form('seats', this.bookFormStep1), 10),
    ).subscribe(() => {
      this.router.navigate(['/profile']);
    });
  }

  ngOnInit(): void {
    this.flightService.readOffer(
      this.bookingService.departure || undefined,
      this.bookingService.arrival || undefined,
    ).subscribe((offers) => {
      this.offers = offers.filter((offer) => offer.departureIcao !== '' && offer.arrivalIcao !== '');
    });

    this.addressService
      .read(this.authService.id)
      .subscribe((addresses) => { this.addresses = addresses; });
  }
}
