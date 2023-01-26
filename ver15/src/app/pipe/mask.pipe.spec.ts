import { MaskPipe } from './mask.pipe';

describe('MaskPipe', () => {
  const pipe = new MaskPipe();
  describe('文字数を指定しなかった場合', () => {
    it('末尾4文字がマスク化されること', () => {
      expect(pipe.transform('12345678')).toBe('1234****');
    });
  });
  describe('文字数を指定した場合', () => {
    it('末尾から指定した文字数分だけマスク化されること', () => {
      expect(pipe.transform('12345678', 2)).toBe('123456**');
    });
  });
});
