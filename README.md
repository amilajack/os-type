# os-type

Bindings to the os_type rust library

## Installation
```bash
npm install os-type
```

## Usage
```js
import osType from 'os-type';

console.log(osType.currentPlatform());
// Object {
//   "osType": "unknown",
//   "version": "0.0.0",
// }
```