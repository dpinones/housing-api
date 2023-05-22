import { MatDialogRef } from '@angular/material/dialog';
import {Component} from '@angular/core';
import {DataService} from '../../services/data.service';
import {Housing} from '../../models/housing';

@Component({
  selector: 'app-add.dialog',
  templateUrl: '../../dialogs/add/add.dialog.html',
  styleUrls: ['../../dialogs/add/add.dialog.css']
})

export class AddDialogComponent {

  data: Housing = new Housing();
  
  constructor(
    public dialogRef: MatDialogRef<AddDialogComponent>,
    public dataService: DataService
  ) {
  }

  submit() {
    // empty stuff

  }

  onNoClick(): void {
    this.dialogRef.close();
  }

  public confirmAdd(): void {
    this.dataService.addHousing(this.data);
  }
}
