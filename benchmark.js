const css = require('lightningcss');
const esbuild = require('esbuild');
const swc = require('./index.js')

let opts = {
  filename: process.argv[process.argv.length - 1],
  code: require('fs').readFileSync(process.argv[process.argv.length - 1]),
  minify: true,
  // source_map: true,
  targets: {
    chrome: 95 << 16
  }
};

async function run() {
  console.time('esbuild');
  let r = await esbuild.transform(opts.code, {
    sourcefile: opts.filename,
    loader: 'css',
    minify: true
  });
  console.timeEnd('esbuild');
  console.log(r.code.length + ' bytes');
  console.log('');

  console.time('lightningcss');
  let res = css.transform(opts);
  console.timeEnd('lightningcss');
  console.log(res.code.length + ' bytes');
  console.log('');

  console.time('swc');
  let r2 = swc.minify(opts.code)
  console.timeEnd('swc');
  console.log(r2.length + ' bytes');
}

run();
