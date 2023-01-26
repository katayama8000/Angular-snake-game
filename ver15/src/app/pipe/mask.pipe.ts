import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'mask',
})
export class MaskPipe implements PipeTransform {
  transform(value: string, _size = 4): string {
    return `${value.slice(0, -_size)}${'*'.repeat(_size)}`;
  }
}
