import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import {
  Address, AddressesService, FlightOfferWithOccupancy, FlightsService,
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
    private flightService: FlightsService,
    private addressService: AddressesService,
    private bookForm: FormBuilder,
    public bookingService: BookingService,
  ) {
    this.bookFormStep1 = this.bookForm.group({
      flight: ['', Validators.required],
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

  ngOnInit(): void {
    this.flightService.readOffer(
      this.bookingService.departure || undefined,
      this.bookingService.arrival || undefined,
    ).subscribe((offers) => {
      this.offers = offers;
    });

    this.addressService
      .read(this.authService.id)
      .subscribe((addresses) => { this.addresses = addresses; });
  }
}
