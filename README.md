# Lz77-wasm

rust lz77 library built with wasm to be usable in nodejs.

## Installation

```shell
npm i @kamyu/lz77
```

## Build

### Requirements

- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [rust](https://www.rust-lang.org/)

### Command

```shell
npm run build
```

## How to use?

> **parameter:** Buffer or Uint8Array  
> **return:** Uint8Array

### Javascript

```javascript
const fs = require('fs');
const lz77 = require('@kamyu/lz77');

lz77.compress(fs.readFileSync('test.txt'));
lz77.decompress(fs.readFileSync('test.txt.lz77'));
```

### Typescript

```typescript
import * as fs from 'fs';
import * as lz77 from '@kamyu/lz77';

lz77.compress(fs.readFileSync('test.txt'));
lz77.decompress(fs.readFileSync('test.txt.lz77'));
```
