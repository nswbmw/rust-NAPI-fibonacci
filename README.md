## rust-NAPI-fibonacci

A fibonacci demo of [rust-node-api](https://github.com/jupp0r/node-api).

### Environment

```
rustc -V => rustc 1.22.0-nightly (0e6f4cf51 2017-09-27)
node -v => v8.3.0
```

```js
const rust = require('./index.node');

function fib (n) {
  if (n === 1 || n === 2) {
    return 1;
  }
  return fib(n - 1) + fib(n - 2);
}

// js
console.time('node');
console.log(fib(40));
console.timeEnd('node');

// rust
console.time('rust');
console.log(rust.fib(40));
console.timeEnd('rust');
```

print

```
102334155
node: 1094.595ms
102334155
rust: 232.569ms
```

### Neon

see [neon-fibonacci](https://github.com/nswbmw/neon-fibonacci).
