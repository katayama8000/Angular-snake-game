import { Component, Inject, Provider } from '@angular/core';
import { BookService } from './book.service';
import { Hoge } from './hoge';

@Component({
  selector: 'app-book',
  templateUrl: './book.component.html',
  styleUrls: ['./book.component.scss'],
  // providers: [
  //   <Provider>{
  //     provide: BookService, //　serviceを注入する際に使うDIトークン
  //     useClass: BookService, //　サービスの作成方法
  //     multi: false, // 同一のDIトークンに対して複数のサービスを登録する場合はtrue
  //   },
  // ],
  providers: [
    {
      provide: Hoge,
      useClass: BookService,
    },
  ],
})
export class BookComponent {
  //constructor(@Inject(BookService) private bookService: BookService) {}
  constructor(@Inject(Hoge) private bookService: BookService) {}

  log() {
    this.bookService.log();
  }
}