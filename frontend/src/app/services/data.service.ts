import {Injectable} from '@angular/core';
import {BehaviorSubject} from 'rxjs';
import {Housing} from '../models/housing';
import {HttpClient, HttpErrorResponse} from '@angular/common/http';

@Injectable()
export class DataService {
  private readonly API_URL = 'http://localhost:8000/api/housings';

  dataChange: BehaviorSubject<Housing[]> = new BehaviorSubject<Housing[]>([]);
  // Temporarily stores data from dialogs
  dialogData: any;

  constructor (private httpClient: HttpClient) {}

  get data(): Housing[] {
    return this.dataChange.value;
  }

  getDialogData() {
    return this.dialogData;
  }

  getAllHousings(): void {
    this.httpClient.get<Housing[]>(this.API_URL).subscribe(data => {
        this.dataChange.next(data);
      },
      (error: HttpErrorResponse) => {
        console.log (error.name + ' ' + error.message);
      });
  }

  addHousing (housing: Housing): void {
    this.httpClient.post(this.API_URL, housing).subscribe(data => {
      this.dialogData = housing;
      // this.toasterService.showToaster('Successfully added', 3000);
      },
      (err: HttpErrorResponse) => {
      // this.toasterService.showToaster('Error occurred. Details: ' + err.name + ' ' + err.message, 8000);
    });
  }

  updateHousing(housing: Housing): void {
    // this.dialogData = housing;
    this.httpClient.put(this.API_URL + '/' + housing.id, housing).subscribe(data => {
      this.dialogData = housing;
      // this.toasterService.showToaster('Successfully edited', 3000);
    },
    (err: HttpErrorResponse) => {
      // this.toasterService.showToaster('Error occurred. Details: ' + err.name + ' ' + err.message, 8000);
    }
  );
  }

  deleteHousing (id: number): void {
    this.httpClient.delete(this.API_URL + '/' + id).subscribe(data => {
      console.log(data['']);
        // this.toasterService.showToaster('Successfully deleted', 3000);
      },
      (err: HttpErrorResponse) => {
        // this.toasterService.showToaster('Error occurred. Details: ' + err.name + ' ' + err.message, 8000);
      }
    );
  }
}
