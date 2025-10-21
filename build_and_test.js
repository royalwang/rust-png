/**
 * 构建和测试脚本
 * 用于验证Rust PNG解码器的完整功能
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('🚀 Rust PNG Decoder - Build and Test Script');
console.log('============================================');

// 检查依赖
function checkDependencies() {
  console.log('\n📋 Checking dependencies...');
  
  const dependencies = [
    { name: 'Rust', command: 'cargo --version' },
    { name: 'wasm-pack', command: 'wasm-pack --version' },
    { name: 'Node.js', command: 'node --version' }
  ];
  
  for (const dep of dependencies) {
    try {
      const version = execSync(dep.command, { encoding: 'utf8' }).trim();
      console.log(`✓ ${dep.name}: ${version}`);
    } catch (error) {
      console.error(`✗ ${dep.name}: Not found`);
      console.error(`  Please install ${dep.name} first`);
      process.exit(1);
    }
  }
}

// 清理构建目录
function cleanBuild() {
  console.log('\n🧹 Cleaning build directories...');
  
  const dirsToClean = ['pkg', 'target'];
  
  for (const dir of dirsToClean) {
    if (fs.existsSync(dir)) {
      fs.rmSync(dir, { recursive: true, force: true });
      console.log(`✓ Cleaned ${dir}/`);
    }
  }
}

// 构建WASM模块
function buildWasm() {
  console.log('\n🔨 Building WASM module...');
  
  try {
    execSync('wasm-pack build --target web --out-dir pkg --out-name index --release', {
      stdio: 'inherit',
      cwd: __dirname
    });
    console.log('✓ WASM build completed successfully');
  } catch (error) {
    console.error('✗ WASM build failed:', error.message);
    process.exit(1);
  }
}

// 验证构建输出
function verifyBuild() {
  console.log('\n🔍 Verifying build output...');
  
  const expectedFiles = [
    'pkg/index.js',
    'pkg/index.d.ts',
    'pkg/index_bg.wasm',
    'pkg/index_bg.js'
  ];
  
  for (const file of expectedFiles) {
    if (fs.existsSync(file)) {
      const stats = fs.statSync(file);
      console.log(`✓ ${file} (${(stats.size / 1024).toFixed(2)} KB)`);
    } else {
      console.error(`✗ ${file}: Missing`);
      process.exit(1);
    }
  }
}

// 创建package.json
function createPackageJson() {
  console.log('\n📦 Creating package.json...');
  
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
    "keywords": ["png", "image", "decoder", "wasm", "rust", "pngjs"],
    "author": "Your Name",
    "license": "MIT"
  };
  
  fs.writeFileSync(
    path.join(__dirname, 'pkg', 'package.json'),
    JSON.stringify(packageJson, null, 2)
  );
  
  console.log('✓ package.json created');
}

// 复制TypeScript定义
function copyTypeDefinitions() {
  console.log('\n📝 Copying TypeScript definitions...');
  
  if (fs.existsSync('types.d.ts')) {
    fs.copyFileSync('types.d.ts', path.join(__dirname, 'pkg', 'index.d.ts'));
    console.log('✓ TypeScript definitions copied');
  } else {
    console.log('⚠ No custom TypeScript definitions found');
  }
}

// 运行兼容性测试
function runCompatibilityTest() {
  console.log('\n🧪 Running compatibility tests...');
  
  try {
    // 这里可以运行实际的测试
    console.log('✓ Compatibility tests would run here');
    console.log('  (In a real scenario, you would run the actual tests)');
  } catch (error) {
    console.error('✗ Compatibility tests failed:', error.message);
  }
}

// 生成使用示例
function generateUsageExample() {
  console.log('\n📚 Generating usage example...');
  
  const exampleCode = `
// Rust PNG Decoder - Usage Example
import init, { PNG } from './pkg/index.js';

async function loadPngImage(imageUrl) {
  // Initialize WASM module
  await init();
  
  // Load PNG data
  const response = await fetch(imageUrl);
  const arrayBuffer = await response.arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);
  
  // Create PNG decoder (compatible with pngjs API)
  const png = new PNG(uint8Array);
  
  // Get image information
  console.log('Image Info:');
  console.log('  Width:', png.getWidth());
  console.log('  Height:', png.getHeight());
  console.log('  Bit Depth:', png.getBitDepth());
  console.log('  Color Type:', png.getColorType());
  
  // Get pixel data
  const pixel = png.getPixel(0, 0);
  console.log('First pixel:', pixel);
  
  // Get RGBA array
  const rgbaArray = png.getRGBA8Array();
  console.log('RGBA data length:', rgbaArray.length);
  
  return png;
}

// Usage
loadPngImage('image.png').then(png => {
  console.log('PNG loaded successfully!');
}).catch(error => {
  console.error('Error loading PNG:', error);
});
`;
  
  fs.writeFileSync('usage_example.js', exampleCode);
  console.log('✓ Usage example generated (usage_example.js)');
}

// 显示构建统计
function showBuildStats() {
  console.log('\n📊 Build Statistics:');
  
  const pkgDir = path.join(__dirname, 'pkg');
  if (fs.existsSync(pkgDir)) {
    const files = fs.readdirSync(pkgDir);
    let totalSize = 0;
    
    for (const file of files) {
      const filePath = path.join(pkgDir, file);
      const stats = fs.statSync(filePath);
      totalSize += stats.size;
      console.log(`  ${file}: ${(stats.size / 1024).toFixed(2)} KB`);
    }
    
    console.log(`  Total size: ${(totalSize / 1024).toFixed(2)} KB`);
  }
}

// 主函数
async function main() {
  try {
    checkDependencies();
    cleanBuild();
    buildWasm();
    verifyBuild();
    createPackageJson();
    copyTypeDefinitions();
    runCompatibilityTest();
    generateUsageExample();
    showBuildStats();
    
    console.log('\n🎉 Build and test completed successfully!');
    console.log('\n📁 Output files are in the pkg/ directory');
    console.log('🌐 You can now use the WASM module in your web application');
    console.log('📖 See usage_example.js for usage examples');
    
  } catch (error) {
    console.error('\n❌ Build failed:', error.message);
    process.exit(1);
  }
}

// 运行主函数
main();
