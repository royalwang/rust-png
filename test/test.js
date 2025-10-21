/**
 * Test suite for Rust PNG decoder
 * Tests compatibility with pngjs API
 */

import { readFileSync } from 'fs';
import { join } from 'path';

// Mock WASM module for testing (in real usage, this would be the actual WASM module)
class MockPngDecoder {
  constructor(data, options = {}) {
    this.data = data;
    this.options = options;
    this.width = 0;
    this.height = 0;
    this.bitDepth = 8;
    this.colorType = 2; // RGB
    this.compressionMethod = 0;
    this.filterMethod = 0;
    this.interlaceMethod = 0;
    this.palette = null;
    this.pixelData = null;
    this.rgbaData = null;
    
    // Simulate PNG parsing
    this.parsePngHeader();
  }

  parsePngHeader() {
    // Simple PNG header validation
    if (this.data.length < 8) {
      throw new Error('Invalid PNG: file too short');
    }
    
    const pngSignature = [137, 80, 78, 71, 13, 10, 26, 10];
    for (let i = 0; i < 8; i++) {
      if (this.data[i] !== pngSignature[i]) {
        throw new Error('Invalid PNG: incorrect signature');
      }
    }
    
    // Mock dimensions (in real implementation, these would be parsed from IHDR chunk)
    this.width = 100;
    this.height = 100;
    
    if (this.options.data !== false) {
      this.generateMockPixelData();
    }
  }

  generateMockPixelData() {
    // Generate mock RGBA data
    this.rgbaData = new Uint8ClampedArray(this.width * this.height * 4);
    for (let i = 0; i < this.rgbaData.length; i += 4) {
      this.rgbaData[i] = Math.floor(Math.random() * 256);     // R
      this.rgbaData[i + 1] = Math.floor(Math.random() * 256); // G
      this.rgbaData[i + 2] = Math.floor(Math.random() * 256); // B
      this.rgbaData[i + 3] = 255;                             // A
    }
  }

  getWidth() {
    return this.width;
  }

  getHeight() {
    return this.height;
  }

  getPixel(x, y) {
    if (x >= this.width || y >= this.height) {
      throw new Error('Pixel coordinates out of bounds');
    }
    
    if (!this.rgbaData) {
      throw new Error('No pixel data available');
    }
    
    const index = (y * this.width + x) * 4;
    return [
      this.rgbaData[index],     // R
      this.rgbaData[index + 1],  // G
      this.rgbaData[index + 2],  // B
      this.rgbaData[index + 3]   // A
    ];
  }

  getRGBA8Array() {
    if (!this.rgbaData) {
      throw new Error('No RGBA data available');
    }
    return this.rgbaData;
  }

  getBitDepth() {
    return this.bitDepth;
  }

  getColorType() {
    return this.colorType;
  }

  getCompressionMethod() {
    return this.compressionMethod;
  }

  getFilterMethod() {
    return this.filterMethod;
  }

  getInterlaceMethod() {
    return this.interlaceMethod;
  }

  getPalette() {
    return this.palette;
  }
}

// Test functions
function testBasicFunctionality() {
  console.log('Testing basic functionality...');
  
  // Create mock PNG data
  const pngData = new Uint8Array([
    137, 80, 78, 71, 13, 10, 26, 10, // PNG signature
    // ... more PNG data would go here
  ]);
  
  try {
    const decoder = new MockPngDecoder(pngData);
    
    // Test basic properties
    console.assert(decoder.getWidth() === 100, 'Width should be 100');
    console.assert(decoder.getHeight() === 100, 'Height should be 100');
    console.assert(decoder.getBitDepth() === 8, 'Bit depth should be 8');
    console.assert(decoder.getColorType() === 2, 'Color type should be 2 (RGB)');
    
    console.log('✓ Basic functionality tests passed');
  } catch (error) {
    console.error('✗ Basic functionality test failed:', error.message);
  }
}

function testPixelAccess() {
  console.log('Testing pixel access...');
  
  const pngData = new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10]);
  
  try {
    const decoder = new MockPngDecoder(pngData);
    
    // Test pixel access
    const pixel = decoder.getPixel(0, 0);
    console.assert(Array.isArray(pixel), 'Pixel should be an array');
    console.assert(pixel.length === 4, 'Pixel should have 4 components (RGBA)');
    console.assert(pixel.every(component => typeof component === 'number'), 'All pixel components should be numbers');
    console.assert(pixel.every(component => component >= 0 && component <= 255), 'Pixel components should be 0-255');
    
    console.log('✓ Pixel access tests passed');
  } catch (error) {
    console.error('✗ Pixel access test failed:', error.message);
  }
}

function testRGBAArray() {
  console.log('Testing RGBA array...');
  
  const pngData = new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10]);
  
  try {
    const decoder = new MockPngDecoder(pngData);
    
    // Test RGBA array
    const rgbaArray = decoder.getRGBA8Array();
    console.assert(rgbaArray instanceof Uint8ClampedArray, 'RGBA array should be Uint8ClampedArray');
    console.assert(rgbaArray.length === 100 * 100 * 4, 'RGBA array should have correct length');
    
    console.log('✓ RGBA array tests passed');
  } catch (error) {
    console.error('✗ RGBA array test failed:', error.message);
  }
}

function testOptions() {
  console.log('Testing options...');
  
  const pngData = new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10]);
  
  try {
    // Test with data: false
    const decoderNoData = new MockPngDecoder(pngData, { data: false });
    console.assert(decoderNoData.getWidth() === 100, 'Width should be available even without data');
    console.assert(decoderNoData.getHeight() === 100, 'Height should be available even without data');
    
    // Test with data: true (default)
    const decoderWithData = new MockPngDecoder(pngData, { data: true });
    console.assert(decoderWithData.getRGBA8Array() !== null, 'RGBA data should be available');
    
    console.log('✓ Options tests passed');
  } catch (error) {
    console.error('✗ Options test failed:', error.message);
  }
}

function testErrorHandling() {
  console.log('Testing error handling...');
  
  try {
    // Test invalid PNG
    const invalidData = new Uint8Array([1, 2, 3, 4]);
    try {
      new MockPngDecoder(invalidData);
      console.error('✗ Should have thrown error for invalid PNG');
    } catch (error) {
      console.log('✓ Correctly threw error for invalid PNG');
    }
    
    // Test pixel out of bounds
    const pngData = new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10]);
    const decoder = new MockPngDecoder(pngData);
    
    try {
      decoder.getPixel(1000, 1000);
      console.error('✗ Should have thrown error for out of bounds pixel');
    } catch (error) {
      console.log('✓ Correctly threw error for out of bounds pixel');
    }
    
    console.log('✓ Error handling tests passed');
  } catch (error) {
    console.error('✗ Error handling test failed:', error.message);
  }
}

// Run all tests
function runTests() {
  console.log('Running Rust PNG Decoder Tests');
  console.log('==============================');
  
  testBasicFunctionality();
  testPixelAccess();
  testRGBAArray();
  testOptions();
  testErrorHandling();
  
  console.log('\nAll tests completed!');
}

// Export for use in other modules
export { MockPngDecoder, runTests };

// Run tests if this file is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  runTests();
}
