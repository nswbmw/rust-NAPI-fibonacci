## rust-NAPI-fibonacci

A fibonacci demo of [rust-node-api](https://github.com/jupp0r/node-api).

### Environment

```
rustc -V => rustc 1.20.0-nightly (229d0d326 2017-06-23)
node -v => v8.1.1
```

```js
const rust = require('./index.node');

function fib(n) {
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
node: 1000.278ms
102334155
rust: 1078.474ms
```

### Neon

see [neon-fibonacci](https://github.com/nswbmw/neon-fibonacci).
