import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { FlightOfferWithOccupancy, FlightsService } from '@skyrocket/ng-api-client';
import { BookingService } from '../_services/booking.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.sass'],
})
export class HomeComponent implements OnInit {
  bookForm: FormGroup;

  private offers: FlightOfferWithOccupancy[] = [];

  constructor(
    private bookingService: BookingService,
    private flightService: FlightsService,
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

  // convenience getter to retrieve all departure airports from flight offerings
  get departures(): string[] {
    return this.offers
      .filter((offer) => (this.form('arrival') ? (this.form('arrival') === offer.arrivalIcao) : true))
      .map((offer) => offer.departureIcao);
  }

  // convenience getter to retrieve all arrival airports from flight offerings
  get arrivals(): string[] {
    return this.offers.map((offer) => offer.arrivalIcao);
  }

  ngOnInit(): void {
    this.flightService.readOffer().subscribe((offers) => {
      this.offers = offers.filter(function (offer) {
        return offer.departureIcao !== '' && offer.arrivalIcao !== ''
      });
    });
  }

  form(control: string): string {
    return this.bookForm.controls[control].value;
  }

  onSubmit(): void {
    this.bookingService.departure = this.form('departure');
    this.bookingService.arrival = this.form('arrival');
    this.bookingService.travelPeriod = {
      dateDeparture: new Date(
        this.bookForm.get(['date', 'dateDeparture'])?.value,
      ),
      dateArrival: new Date(this.bookForm.get(['date', 'dateArrival'])?.value),
    };
    this.router.navigate(['/book']);
  }
}
