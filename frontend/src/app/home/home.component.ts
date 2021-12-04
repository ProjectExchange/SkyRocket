import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { BookingService } from '../_services/booking.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.sass'],
})
export class HomeComponent implements OnInit {
  bookForm: FormGroup;

  constructor(
    private bookingService: BookingService,
    private formBuilder: FormBuilder,
    private router: Router,
  ) {
    this.bookForm = this.formBuilder.group({
      departure: ['', [Validators.required.bind(this)]],
      arrival: ['', [Validators.required.bind(this)]],
      date: this.formBuilder.group({
        dateDeparture: '',
        dateArrival: '',
      }),
    });
  }

  ngOnInit(): void {}

  form(control: string): string {
    return this.bookForm.controls[control].value;
  }

  onSubmit(): void {
    this.bookingService.departure = this.form('departure');
    this.bookingService.arrival = this.form('arrival');
    this.bookingService.travelPeriod = {
      dateDeparture: new Date(this.bookForm.get(['date', 'dateDeparture'])?.value),
      dateArrival: new Date(this.bookForm.get(['date', 'dateArrival'])?.value)
    };
    this.router.navigate(['/book']);
  }
}
