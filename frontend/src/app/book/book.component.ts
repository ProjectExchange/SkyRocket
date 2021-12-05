import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { AuthService } from '../_services/auth.service';
import { BookingService } from '../_services/booking.service';

interface Flight {
  value: string;
  viewValue: string;
}

interface Address {
  value: string;
  viewValue: string;
}

@Component({
  selector: 'app-book',
  templateUrl: './book.component.html',
  styleUrls: ['./book.component.sass'],
})
export class BookComponent {
  departure: string = this.bookingService.departure;
  arrival: string = this.bookingService.arrival;
  dateDeparture: string = this.bookingService.dateDeparture.toString().split(' ').slice(1, 4).join(' ');
  dateArrival: string = this.bookingService.dateArrival.toString().split(' ').slice(1, 4).join(' ');

  bookFormStep1: FormGroup;
  bookFormStep2: FormGroup;
  bookFormStep3: FormGroup;

  flights: Flight[] = [
    {value: '0', viewValue: 'Heilbronn → Mannheim 12:00 Uhr'},
    {value: '1', viewValue: 'Heilbronn → Mannheim 15:00 Uhr'},
    {value: '2', viewValue: 'Heilbronn → Mannheim 21:42 Uhr'},
  ];
  addresses: Address[] = [
    {value: '0', viewValue: '2020 Burwell Heights Road, Galveston, TX 77553'},
    {value: '1', viewValue: 'Buelowstrasse 92, 56472 Hahn bei Marienberg '},
    {value: '2', viewValue: 'Herentalsebaan 299, 1190 Brussel '},
  ];

  constructor(
    private authService: AuthService,
    private bookingService: BookingService,
    private bookForm: FormBuilder,
  ) {
    this.bookFormStep1 = this.bookForm.group({
      flight: ['', Validators.required],
    });

    this.bookFormStep2 = this.bookForm.group({
      firstname: [
        this.authService.firstname,
        [Validators.required.bind(this)]
      ],
      lastname: [
        this.authService.lastname,
        [Validators.required.bind(this)]
      ]
    });

    this.bookFormStep3 = this.bookForm.group({
      address: ['', Validators.required],
    });
  }
}
