# gyromagnetic-ratio

[![NPM version][npm-image]][npm-url]
[![build status][ci-image]][ci-url]
[![Test coverage][codecov-image]][codecov-url]
[![npm download][download-image]][download-url]

Reference values for gyromagnetic ratio in $rad \over Ts $ extracted from:
- [NMR nomenclature. Nuclear spin properties and conventions for chemical shifts(IUPAC Recommendations 2001)](https://dx.doi.org/10.1351/pac200173111795).

## Installation

`$ npm i gyromagnetic-ratio`

## Usage

```js
import { gyromagneticRatio } from 'gyromagnetic-ratio';

const proton = gyromagneticRatio['1H'];

console.log(proton);
// 267522187.44
```

## License

[MIT](./LICENSE)

[npm-image]: https://img.shields.io/npm/v/gyromagnetic-ratio.svg
[npm-url]: https://www.npmjs.com/package/gyromagnetic-ratio
[ci-image]: https://github.com/cheminfo/gyromagnetic-ratio/workflows/Node.js%20CI/badge.svg?branch=main
[ci-url]: https://github.com/cheminfo/gyromagnetic-ratio/actions?query=workflow%3A%22Node.js+CI%22
[codecov-image]: https://img.shields.io/codecov/c/github/cheminfo/gyromagnetic-ratio.svg
[codecov-url]: https://codecov.io/gh/cheminfo/gyromagnetic-ratio
[download-image]: https://img.shields.io/npm/dm/gyromagnetic-ratio.svg
[download-url]: https://www.npmjs.com/package/gyromagnetic-ratio
