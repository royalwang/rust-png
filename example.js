/**
 * Example usage of Rust PNG Decoder
 * This demonstrates how to use the WASM module in a web application
 */

// Example: Loading and displaying a PNG image
async function loadAndDisplayPng(imageUrl) {
  try {
    // Load the PNG file
    const response = await fetch(imageUrl);
    const arrayBuffer = await response.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);
    
    // Import the WASM module (in real usage, this would be the actual WASM module)
    // import init, { PngDecoder } from './pkg/index.js';
    // await init();
    
    // For this example, we'll use a mock implementation
    const PngDecoder = MockPngDecoder;
    
    // Create decoder with options
    const options = { data: true };
    const decoder = new PngDecoder(uint8Array, options);
    
    // Get image information
    console.log('Image Information:');
    console.log(`  Width: ${decoder.getWidth()}`);
    console.log(`  Height: ${decoder.getHeight()}`);
    console.log(`  Bit Depth: ${decoder.getBitDepth()}`);
    console.log(`  Color Type: ${decoder.getColorType()}`);
    console.log(`  Compression Method: ${decoder.getCompressionMethod()}`);
    console.log(`  Filter Method: ${decoder.getFilterMethod()}`);
    console.log(`  Interlace Method: ${decoder.getInterlaceMethod()}`);
    
    // Get palette if available
    const palette = decoder.getPalette();
    if (palette) {
      console.log(`  Palette: ${palette.length} bytes`);
    }
    
    // Get pixel data
    const rgbaArray = decoder.getRGBA8Array();
    console.log(`  RGBA Data: ${rgbaArray.length} bytes`);
    
    // Get specific pixel
    const pixel = decoder.getPixel(0, 0);
    console.log(`  First pixel (0,0): R=${pixel[0]}, G=${pixel[1]}, B=${pixel[2]}, A=${pixel[3]}`);
    
    // Render to canvas
    const canvas = document.getElementById('canvas');
    if (canvas) {
      const ctx = canvas.getContext('2d');
      canvas.width = decoder.getWidth();
      canvas.height = decoder.getHeight();
      
      const imageData = ctx.createImageData(decoder.getWidth(), decoder.getHeight());
      imageData.data.set(rgbaArray);
      ctx.putImageData(imageData, 0, 0);
    }
    
    return decoder;
  } catch (error) {
    console.error('Error loading PNG:', error);
    throw error;
  }
}

// Example: Processing multiple PNG files
async function processPngFiles(fileList) {
  const results = [];
  
  for (const file of fileList) {
    try {
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      
      // Create decoder without pixel data for faster processing
      const decoder = new MockPngDecoder(uint8Array, { data: false });
      
      results.push({
        name: file.name,
        width: decoder.getWidth(),
        height: decoder.getHeight(),
        bitDepth: decoder.getBitDepth(),
        colorType: decoder.getColorType()
      });
    } catch (error) {
      console.error(`Error processing ${file.name}:`, error);
      results.push({
        name: file.name,
        error: error.message
      });
    }
  }
  
  return results;
}

// Example: Batch pixel processing
function processPixels(decoder, processor) {
  const width = decoder.getWidth();
  const height = decoder.getHeight();
  const results = [];
  
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      try {
        const pixel = decoder.getPixel(x, y);
        const processed = processor(pixel, x, y);
        results.push(processed);
      } catch (error) {
        console.error(`Error processing pixel (${x}, ${y}):`, error);
      }
    }
  }
  
  return results;
}

// Example: Find all pixels of a specific color
function findPixelsByColor(decoder, targetColor) {
  const [targetR, targetG, targetB, targetA] = targetColor;
  const matches = [];
  
  const width = decoder.getWidth();
  const height = decoder.getHeight();
  
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      try {
        const pixel = decoder.getPixel(x, y);
        if (pixel[0] === targetR && pixel[1] === targetG && 
            pixel[2] === targetB && pixel[3] === targetA) {
          matches.push({ x, y });
        }
      } catch (error) {
        // Skip invalid pixels
      }
    }
  }
  
  return matches;
}

// Example: Convert to grayscale
function convertToGrayscale(decoder) {
  const width = decoder.getWidth();
  const height = decoder.getHeight();
  const grayscaleData = new Uint8ClampedArray(width * height * 4);
  
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      try {
        const pixel = decoder.getPixel(x, y);
        const gray = Math.round(0.299 * pixel[0] + 0.587 * pixel[1] + 0.114 * pixel[2]);
        
        const index = (y * width + x) * 4;
        grayscaleData[index] = gray;     // R
        grayscaleData[index + 1] = gray; // G
        grayscaleData[index + 2] = gray; // B
        grayscaleData[index + 3] = pixel[3]; // A
      } catch (error) {
        // Skip invalid pixels
      }
    }
  }
  
  return grayscaleData;
}

// Export functions for use in other modules
export {
  loadAndDisplayPng,
  processPngFiles,
  processPixels,
  findPixelsByColor,
  convertToGrayscale
};

// Mock implementation for demonstration
class MockPngDecoder {
  constructor(data, options = {}) {
    this.data = data;
    this.options = options;
    this.width = 100;
    this.height = 100;
    this.bitDepth = 8;
    this.colorType = 2;
    this.compressionMethod = 0;
    this.filterMethod = 0;
    this.interlaceMethod = 0;
    this.palette = null;
    this.rgbaData = null;
    
    if (this.options.data !== false) {
      this.generateMockData();
    }
  }
  
  generateMockData() {
    this.rgbaData = new Uint8ClampedArray(this.width * this.height * 4);
    for (let i = 0; i < this.rgbaData.length; i += 4) {
      this.rgbaData[i] = Math.floor(Math.random() * 256);
      this.rgbaData[i + 1] = Math.floor(Math.random() * 256);
      this.rgbaData[i + 2] = Math.floor(Math.random() * 256);
      this.rgbaData[i + 3] = 255;
    }
  }
  
  getWidth() { return this.width; }
  getHeight() { return this.height; }
  getBitDepth() { return this.bitDepth; }
  getColorType() { return this.colorType; }
  getCompressionMethod() { return this.compressionMethod; }
  getFilterMethod() { return this.filterMethod; }
  getInterlaceMethod() { return this.interlaceMethod; }
  getPalette() { return this.palette; }
  
  getPixel(x, y) {
    if (x >= this.width || y >= this.height) {
      throw new Error('Pixel coordinates out of bounds');
    }
    if (!this.rgbaData) {
      throw new Error('No pixel data available');
    }
    const index = (y * this.width + x) * 4;
    return [
      this.rgbaData[index],
      this.rgbaData[index + 1],
      this.rgbaData[index + 2],
      this.rgbaData[index + 3]
    ];
  }
  
  getRGBA8Array() {
    if (!this.rgbaData) {
      throw new Error('No RGBA data available');
    }
    return this.rgbaData;
  }
}
