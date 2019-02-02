# os-type

Bindings to the [os_type](https://github.com/schultyy/os_type) Rust library using [Neon](https://github.com/neon-bindings/neon)

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
