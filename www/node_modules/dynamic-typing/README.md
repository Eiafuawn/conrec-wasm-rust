# dynamic-typing

[![NPM version][npm-image]][npm-url]
[![build status][ci-image]][ci-url]
[![Test coverage][codecov-image]][codecov-url]
[![npm download][download-image]][download-url]
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.5482823.svg)](https://doi.org/10.5281/zenodo.5482823)

## Dynamically types a string.

When parsing text files, like the ones coming out fromm scientific instruments,
it is often useful to convert the obtained strings to their corresponding types.

Indeed when you parse the file while all the fields are text some of them represents in fact numbers or booleans.

This package will try to make the conversion for you using the following rules:

- a string that has as lowercase value 'true' or 'false' will be converted to the corresponding boolean
- a string that when converted to number (using `Number(string)`) does not yield to NaN will be converted to number
- other strings will be kept as string

This package was optimized for speed and seems to work pretty well. Please don't hesitate to submit bug reports or contribute.

## Installation

`$ npm i dynamic-typing`

## Usage

```js
import { parseString } from 'dynamic-typing';

const result = parseString('0x100');

// result is the number 256
```

More examples:

- `parseString('')` ➡ `''`
- `parseString(' ')` ➡ `' '`
- `parseString('.')` ➡ `'.'`
- `parseString('0.')` ➡ `0`
- `parseString('0.0')` ➡ `0`
- `parseString('abc')` ➡ `'abc'`
- `parseString('True')` ➡ `true`
- `parseString('false')` ➡ `false`
- `parseString('0')` ➡ `0`
- `parseString('123')` ➡ `123`
- `parseString('123.456')` ➡ `123.456`
- `parseString('12e3')` ➡ `12000`
- `parseString('0x10')` ➡ `16`
- `parseString('0b10')` ➡ `2`

## [API Documentation](https://cheminfo.github.io/dynamic-typing/)

## License

[MIT](./LICENSE)

[npm-image]: https://img.shields.io/npm/v/dynamic-typing.svg
[npm-url]: https://www.npmjs.com/package/dynamic-typing
[ci-image]: https://github.com/cheminfo/dynamic-typing/workflows/Node.js%20CI/badge.svg?branch=main
[ci-url]: https://github.com/cheminfo/dynamic-typing/actions?query=workflow%3A%22Node.js+CI%22
[codecov-image]: https://img.shields.io/codecov/c/github/cheminfo/dynamic-typing.svg
[codecov-url]: https://codecov.io/gh/cheminfo/dynamic-typing
[download-image]: https://img.shields.io/npm/dm/dynamic-typing.svg
[download-url]: https://www.npmjs.com/package/dynamic-typing
