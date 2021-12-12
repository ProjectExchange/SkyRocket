import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ReactiveFormsModule } from '@angular/forms';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { Router } from '@angular/router';
import { AddressesService, FlightsService } from '@skyrocket/ng-api-client';

import { BookComponent } from './book.component';

describe('BookComponent', () => {
  let component: BookComponent;
  let fixture: ComponentFixture<BookComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [BookComponent],
      imports: [
        BrowserAnimationsModule,
        MatInputModule,
        MatSelectModule,
        ReactiveFormsModule,
      ],
      providers: [
        { provide: AddressesService, useValue: AddressesService },
        { provide: FlightsService, useValue: FlightsService },
        { provide: Router, useValue: Router },
      ],
    }).compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BookComponent);
    component = fixture.componentInstance;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
