import { UserService } from './user.service';

export class AliasService extends UserService {
  constructor() {
    super();
  }
  override show() {
    return 'alias: ' + super.show();
  }
}
