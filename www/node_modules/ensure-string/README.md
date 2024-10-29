# ensure-string

[![NPM version][npm-image]][npm-url]
[![build status][ci-image]][ci-url]
[![Test coverage][codecov-image]][codecov-url]
[![npm download][download-image]][download-url]

If required, convert the input value to a string.

The input value may be a Buffer, ArrayBuffer, Int8Array.

By default the conversion will consider that the text encoding is `utf-8`.
If the file contains an UTF-16 byte-order mark (BOM), this will be detected and default to `utf-16le` or `utf-16be` accordingly.

If the input value is a text, no change is done. The method returns the value without further processing.

## Installation

`$ npm i ensure-string`

## Usage

We have a `text.txt` file containing the string `ABC`.

```js
import { ensureString } from '..';
import { readFileSync } from 'fs';
import { join } from 'path';
const blob = readFileSync(join(__dirname, 'test.txt')); // read the file as an ArrayBuffer
const text = ensureString(blob);
console.log(text); // the text is a string containing ÀBC`
```

## [API Documentation](https://cheminfo.github.io/ensure-string/)

## License

[MIT](./LICENSE)

[npm-image]: https://img.shields.io/npm/v/ensure-string.svg
[npm-url]: https://www.npmjs.com/package/ensure-string
[ci-image]: https://github.com/cheminfo/ensure-string/workflows/Node.js%20CI/badge.svg?branch=main
[ci-url]: https://github.com/cheminfo/ensure-string/actions?query=workflow%3A%22Node.js+CI%22
[codecov-image]: https://img.shields.io/codecov/c/github/cheminfo/ensure-string.svg
[codecov-url]: https://codecov.io/gh/cheminfo/ensure-string
[download-image]: https://img.shields.io/npm/dm/ensure-string.svg
[download-url]: https://www.npmjs.com/package/ensure-string
