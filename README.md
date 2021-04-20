# chinese_checkers

```json
   // Code for coloring
```
```html
   
```
```js
   // Code for coloring
```
```css
   span.red {
      color: red
   }
```
import { Component } from '@angular/core';
import { MovieService } from './services/movie.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
  providers: [ MovieService ]
})
export class AppComponent {
  title = 'app works!';
}
