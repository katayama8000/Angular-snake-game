import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class BookService {
  constructor() {}

  log() {
    console.log('BookService');
  }
}
