import { Injectable } from '@angular/core';

interface TravelPeriod {
  dateDeparture: Date;
  dateArrival: Date;
}

export interface Booking {
  departure: string;
  arrival: string;
  travelPeriod: TravelPeriod;
}

@Injectable({
  providedIn: 'root',
})
export class BookingService implements Booking {
  #departure = '';

  #arrival = '';

  #travelPeriod: TravelPeriod = {
    dateDeparture: new Date(),
    dateArrival: new Date(),
  };

  constructor() { /**/ }

  set departure(departure: string) {
    this.#departure = departure;
  }

  get departure(): string {
    return this.#departure || '';
  }

  set arrival(arrival: string) {
    this.#arrival = arrival;
  }

  get arrival(): string {
    return this.#arrival || '';
  }

  set travelPeriod(travelPeriod: TravelPeriod) {
    this.#travelPeriod = travelPeriod;
  }

  get dateDeparture(): Date {
    return this.#travelPeriod.dateDeparture;
  }

  get dateArrival(): Date {
    return this.#travelPeriod.dateArrival;
  }
}
