const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('Building Rust PNG decoder for WASM...');

// 检查是否安装了wasm-pack
try {
  execSync('wasm-pack --version', { stdio: 'ignore' });
} catch (error) {
  console.error('wasm-pack is not installed. Please install it first:');
  console.error('npm install -g wasm-pack');
  process.exit(1);
}

// 构建WASM模块
console.log('Building WASM module...');
try {
  execSync('wasm-pack build --target web --out-dir pkg --out-name index', { 
    stdio: 'inherit',
    cwd: __dirname 
  });
  console.log('WASM build completed successfully!');
} catch (error) {
  console.error('WASM build failed:', error.message);
  process.exit(1);
}

// 创建package.json for the pkg directory
const packageJson = {
  "name": "rust-png",
  "version": "0.1.0",
  "description": "A Rust implementation of pngjs with WASM support",
  "main": "index.js",
  "types": "index.d.ts",
  "files": [
    "index.js",
    "index.d.ts",
    "index_bg.wasm",
    "index_bg.js"
  ],
  "keywords": ["png", "image", "decoder", "wasm", "rust"],
  "author": "Your Name",
  "license": "MIT"
};

fs.writeFileSync(path.join(__dirname, 'pkg', 'package.json'), JSON.stringify(packageJson, null, 2));

// 复制TypeScript定义文件
fs.copyFileSync(
  path.join(__dirname, 'types.d.ts'),
  path.join(__dirname, 'pkg', 'index.d.ts')
);

console.log('Build completed! Output files are in the pkg/ directory.');
console.log('You can now use the WASM module in your web application.');
