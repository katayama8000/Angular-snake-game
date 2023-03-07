import { Component, Provider } from '@angular/core';
import { BookService } from './book.service';

@Component({
  selector: 'app-book',
  templateUrl: './book.component.html',
  styleUrls: ['./book.component.scss'],
  providers: [
    <Provider>{
      provide: BookService,
      useClass: BookService,
      multi: false,
    },
  ],
})
export class BookComponent {
  constructor(private bookService: BookService) {}

  log() {
    this.bookService.log();
  }
}
