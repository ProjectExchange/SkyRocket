import { ComponentFixture, TestBed } from '@angular/core/testing';
import { RouterTestingModule } from '@angular/router/testing';
import { LoginService } from '@skyrocket/ng-api-client';

import { OauthComponent } from './oauth.component';

describe('OauthComponent', () => {
  let component: OauthComponent;
  let fixture: ComponentFixture<OauthComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [OauthComponent],
      imports: [RouterTestingModule],
      providers: [{ provide: LoginService, useValue: LoginService }],
    }).compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(OauthComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
