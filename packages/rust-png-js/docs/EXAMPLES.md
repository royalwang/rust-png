# Rust PNG JS - 示例代码

## 目录

- [浏览器示例](#浏览器示例)
- [Node.js示例](#nodejs示例)
- [React示例](#react示例)
- [Vue示例](#vue示例)
- [Angular示例](#angular示例)
- [Web Workers示例](#web-workers示例)
- [批量处理示例](#批量处理示例)
- [高级功能示例](#高级功能示例)

## 浏览器示例

### 基本文件上传

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>PNG处理示例</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .container { max-width: 800px; margin: 0 auto; }
        .file-input { margin: 20px 0; }
        .canvas-container { text-align: center; margin: 20px 0; }
        canvas { border: 1px solid #ddd; max-width: 100%; }
        .info { background: #f8f9fa; padding: 15px; border-radius: 5px; margin: 10px 0; }
        .error { background: #f8d7da; color: #721c24; }
        .success { background: #d4edda; color: #155724; }
    </style>
</head>
<body>
    <div class="container">
        <h1>PNG处理示例</h1>
        
        <div class="file-input">
            <input type="file" id="fileInput" accept=".png">
        </div>
        
        <div id="status" class="info" style="display: none;">
            <div id="statusText"></div>
        </div>
        
        <div class="canvas-container">
            <canvas id="canvas" style="display: none;"></canvas>
            <p id="noImageText">请选择PNG文件</p>
        </div>
        
        <div id="imageInfo" class="info" style="display: none;">
            <h3>图像信息</h3>
            <div id="infoContent"></div>
        </div>
    </div>

    <script type="module">
        import { PNG, PNGSync, validatePNG, getPNGInfo } from './dist/index.js';

        const fileInput = document.getElementById('fileInput');
        const canvas = document.getElementById('canvas');
        const noImageText = document.getElementById('noImageText');
        const status = document.getElementById('status');
        const statusText = document.getElementById('statusText');
        const imageInfo = document.getElementById('imageInfo');
        const infoContent = document.getElementById('infoContent');

        function showStatus(message, type = 'info') {
            statusText.textContent = message;
            status.className = `info ${type}`;
            status.style.display = 'block';
        }

        fileInput.addEventListener('change', async (event) => {
            const file = event.target.files[0];
            if (!file) return;

            showStatus('正在处理文件...', 'info');

            try {
                // 读取文件数据
                const data = new Uint8Array(await file.arrayBuffer());
                
                // 验证PNG
                const isValid = await validatePNG(data);
                if (!isValid) {
                    throw new Error('无效的PNG文件');
                }

                // 获取PNG信息
                const info = await getPNGInfo(data);
                if (!info) {
                    throw new Error('无法获取PNG信息');
                }

                // 解析PNG
                const pngSync = new PNGSync();
                const png = pngSync.read(data);

                // 显示图像
                const ctx = canvas.getContext('2d');
                canvas.width = png.width;
                canvas.height = png.height;

                const imageData = ctx.createImageData(png.width, png.height);
                imageData.data.set(png.getRGBA8Array());
                ctx.putImageData(imageData, 0, 0);

                canvas.style.display = 'block';
                noImageText.style.display = 'none';

                // 显示图像信息
                infoContent.innerHTML = `
                    <div><strong>文件名:</strong> ${file.name}</div>
                    <div><strong>文件大小:</strong> ${(file.size / 1024).toFixed(2)} KB</div>
                    <div><strong>图像尺寸:</strong> ${png.width} x ${png.height}</div>
                    <div><strong>颜色类型:</strong> ${png.getColorType()}</div>
                    <div><strong>位深度:</strong> ${png.getBitDepth()}</div>
                    <div><strong>压缩方法:</strong> ${png.getCompressionMethod()}</div>
                    <div><strong>滤镜方法:</strong> ${png.getFilterMethod()}</div>
                    <div><strong>交错方法:</strong> ${png.getInterlaceMethod()}</div>
                    <div><strong>Gamma值:</strong> ${png.gamma}</div>
                    <div><strong>Alpha通道:</strong> ${png.alpha ? '是' : '否'}</div>
                `;
                imageInfo.style.display = 'block';

                showStatus('文件处理成功！', 'success');

            } catch (error) {
                showStatus(`处理失败: ${error.message}`, 'error');
                console.error('处理错误:', error);
            }
        });
    </script>
</body>
</html>
```

### 图像编辑器

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>PNG图像编辑器</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .container { max-width: 1200px; margin: 0 auto; }
        .toolbar { margin: 20px 0; }
        .button { background: #007bff; color: white; border: none; padding: 10px 20px; margin: 5px; border-radius: 4px; cursor: pointer; }
        .button:hover { background: #0056b3; }
        .button:disabled { background: #ccc; cursor: not-allowed; }
        .canvas-container { text-align: center; margin: 20px 0; }
        canvas { border: 1px solid #ddd; max-width: 100%; }
        .controls { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin: 20px 0; }
        .control-group { background: #f8f9fa; padding: 15px; border-radius: 5px; }
        .slider { width: 100%; margin: 10px 0; }
        .info { background: #e9ecef; padding: 10px; border-radius: 4px; margin: 10px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>PNG图像编辑器</h1>
        
        <div class="toolbar">
            <input type="file" id="fileInput" accept=".png">
            <button id="saveBtn" class="button" disabled>保存PNG</button>
            <button id="resetBtn" class="button" disabled>重置</button>
        </div>
        
        <div class="controls">
            <div class="control-group">
                <h3>滤镜效果</h3>
                <button id="grayscaleBtn" class="button">灰度化</button>
                <button id="invertBtn" class="button">反色</button>
                <button id="brightnessBtn" class="button">亮度调整</button>
                <button id="contrastBtn" class="button">对比度调整</button>
            </div>
            
            <div class="control-group">
                <h3>图像调整</h3>
                <label>亮度: <span id="brightnessValue">0</span></label>
                <input type="range" id="brightnessSlider" class="slider" min="-100" max="100" value="0">
                
                <label>对比度: <span id="contrastValue">0</span></label>
                <input type="range" id="contrastSlider" class="slider" min="-100" max="100" value="0">
                
                <label>Gamma: <span id="gammaValue">1.0</span></label>
                <input type="range" id="gammaSlider" class="slider" min="0.1" max="3.0" step="0.1" value="1.0">
            </div>
        </div>
        
        <div class="canvas-container">
            <canvas id="canvas" style="display: none;"></canvas>
            <p id="noImageText">请选择PNG文件</p>
        </div>
        
        <div id="imageInfo" class="info" style="display: none;">
            <div id="infoContent"></div>
        </div>
    </div>

    <script type="module">
        import { PNG, PNGSync, optimizePNG } from './dist/index.js';

        let currentPNG = null;
        let originalData = null;
        let currentData = null;

        const fileInput = document.getElementById('fileInput');
        const saveBtn = document.getElementById('saveBtn');
        const resetBtn = document.getElementById('resetBtn');
        const canvas = document.getElementById('canvas');
        const noImageText = document.getElementById('noImageText');
        const imageInfo = document.getElementById('imageInfo');
        const infoContent = document.getElementById('infoContent');

        // 滤镜按钮
        const grayscaleBtn = document.getElementById('grayscaleBtn');
        const invertBtn = document.getElementById('invertBtn');
        const brightnessBtn = document.getElementById('brightnessBtn');
        const contrastBtn = document.getElementById('contrastBtn');

        // 滑块
        const brightnessSlider = document.getElementById('brightnessSlider');
        const contrastSlider = document.getElementById('contrastSlider');
        const gammaSlider = document.getElementById('gammaSlider');
        const brightnessValue = document.getElementById('brightnessValue');
        const contrastValue = document.getElementById('contrastValue');
        const gammaValue = document.getElementById('gammaValue');

        function updateImage() {
            if (!currentPNG) return;

            const ctx = canvas.getContext('2d');
            const imageData = ctx.createImageData(currentPNG.width, currentPNG.height);
            imageData.data.set(currentData);
            ctx.putImageData(imageData, 0, 0);
        }

        function applyGrayscale() {
            if (!currentData) return;

            for (let i = 0; i < currentData.length; i += 4) {
                const gray = currentData[i] * 0.299 + currentData[i + 1] * 0.587 + currentData[i + 2] * 0.114;
                currentData[i] = gray;     // R
                currentData[i + 1] = gray; // G
                currentData[i + 2] = gray; // B
            }
            updateImage();
        }

        function applyInvert() {
            if (!currentData) return;

            for (let i = 0; i < currentData.length; i += 4) {
                currentData[i] = 255 - currentData[i];         // R
                currentData[i + 1] = 255 - currentData[i + 1]; // G
                currentData[i + 2] = 255 - currentData[i + 2]; // B
            }
            updateImage();
        }

        function applyBrightness(brightness) {
            if (!currentData) return;

            for (let i = 0; i < currentData.length; i += 4) {
                currentData[i] = Math.max(0, Math.min(255, currentData[i] + brightness));         // R
                currentData[i + 1] = Math.max(0, Math.min(255, currentData[i + 1] + brightness)); // G
                currentData[i + 2] = Math.max(0, Math.min(255, currentData[i + 2] + brightness)); // B
            }
            updateImage();
        }

        function applyContrast(contrast) {
            if (!currentData) return;

            const factor = (259 * (contrast + 255)) / (255 * (259 - contrast));
            
            for (let i = 0; i < currentData.length; i += 4) {
                currentData[i] = Math.max(0, Math.min(255, factor * (currentData[i] - 128) + 128));         // R
                currentData[i + 1] = Math.max(0, Math.min(255, factor * (currentData[i + 1] - 128) + 128)); // G
                currentData[i + 2] = Math.max(0, Math.min(255, factor * (currentData[i + 2] - 128) + 128)); // B
            }
            updateImage();
        }

        function applyGamma(gamma) {
            if (!currentData) return;

            for (let i = 0; i < currentData.length; i += 4) {
                currentData[i] = Math.max(0, Math.min(255, Math.pow(currentData[i] / 255, gamma) * 255));         // R
                currentData[i + 1] = Math.max(0, Math.min(255, Math.pow(currentData[i + 1] / 255, gamma) * 255)); // G
                currentData[i + 2] = Math.max(0, Math.min(255, Math.pow(currentData[i + 2] / 255, gamma) * 255)); // B
            }
            updateImage();
        }

        function updateInfo() {
            if (!currentPNG) return;

            infoContent.innerHTML = `
                <div><strong>图像尺寸:</strong> ${currentPNG.width} x ${currentPNG.height}</div>
                <div><strong>颜色类型:</strong> ${currentPNG.getColorType()}</div>
                <div><strong>位深度:</strong> ${currentPNG.getBitDepth()}</div>
                <div><strong>Alpha通道:</strong> ${currentPNG.alpha ? '是' : '否'}</div>
            `;
            imageInfo.style.display = 'block';
        }

        // 文件输入处理
        fileInput.addEventListener('change', async (event) => {
            const file = event.target.files[0];
            if (!file) return;

            try {
                const data = new Uint8Array(await file.arrayBuffer());
                const pngSync = new PNGSync();
                currentPNG = pngSync.read(data);
                originalData = new Uint8Array(currentPNG.getRGBA8Array());
                currentData = new Uint8Array(originalData);

                // 显示图像
                canvas.width = currentPNG.width;
                canvas.height = currentPNG.height;
                canvas.style.display = 'block';
                noImageText.style.display = 'none';

                updateImage();
                updateInfo();

                saveBtn.disabled = false;
                resetBtn.disabled = false;

            } catch (error) {
                console.error('处理文件失败:', error);
                alert('处理文件失败: ' + error.message);
            }
        });

        // 滤镜按钮事件
        grayscaleBtn.addEventListener('click', applyGrayscale);
        invertBtn.addEventListener('click', applyInvert);

        // 滑块事件
        brightnessSlider.addEventListener('input', (e) => {
            const value = parseInt(e.target.value);
            brightnessValue.textContent = value;
            applyBrightness(value);
        });

        contrastSlider.addEventListener('input', (e) => {
            const value = parseInt(e.target.value);
            contrastValue.textContent = value;
            applyContrast(value);
        });

        gammaSlider.addEventListener('input', (e) => {
            const value = parseFloat(e.target.value);
            gammaValue.textContent = value.toFixed(1);
            applyGamma(value);
        });

        // 保存按钮
        saveBtn.addEventListener('click', async () => {
            if (!currentPNG) return;

            try {
                // 创建新的PNG对象
                const newPNG = new PNG();
                newPNG.parseSync(originalData);
                
                // 应用当前数据
                (newPNG as any)._data = currentData;
                
                // 优化PNG
                const optimizedData = await optimizePNG(newPNG.pack(), {
                    deflateLevel: 9,
                    filterType: 5
                });

                // 下载文件
                const blob = new Blob([optimizedData], { type: 'image/png' });
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'edited_image.png';
                a.click();
                URL.revokeObjectURL(url);

            } catch (error) {
                console.error('保存失败:', error);
                alert('保存失败: ' + error.message);
            }
        });

        // 重置按钮
        resetBtn.addEventListener('click', () => {
            if (!originalData) return;

            currentData = new Uint8Array(originalData);
            updateImage();

            // 重置滑块
            brightnessSlider.value = '0';
            contrastSlider.value = '0';
            gammaSlider.value = '1.0';
            brightnessValue.textContent = '0';
            contrastValue.textContent = '0';
            gammaValue.textContent = '1.0';
        });
    </script>
</body>
</html>
```

## Node.js示例

### 基本文件处理

```javascript
const { PNG, PNGSync, validatePNG, getPNGInfo, optimizePNG } = require('rust-png-js');
const fs = require('fs');
const path = require('path');

async function processPNGFile(inputPath, outputPath) {
    try {
        console.log('开始处理PNG文件...');
        
        // 读取PNG文件
        const data = fs.readFileSync(inputPath);
        console.log('文件大小:', data.length, 'bytes');
        
        // 验证PNG
        const isValid = await validatePNG(data);
        if (!isValid) {
            throw new Error('无效的PNG文件');
        }
        console.log('PNG文件验证通过');
        
        // 获取PNG信息
        const info = await getPNGInfo(data);
        if (info) {
            console.log('PNG信息:');
            console.log('  尺寸:', info.dimensions.width, 'x', info.dimensions.height);
            console.log('  颜色类型:', info.colorInfo.colorType);
            console.log('  位深度:', info.colorInfo.bitDepth);
            console.log('  压缩方法:', info.compression.method);
            console.log('  滤镜方法:', info.compression.filter);
            console.log('  交错方法:', info.interlace.method);
        }
        
        // 解析PNG
        const pngSync = new PNGSync();
        const png = pngSync.read(data);
        console.log('PNG解析成功');
        
        // 优化PNG
        console.log('开始优化PNG...');
        const optimizedData = await optimizePNG(data, {
            deflateLevel: 9,
            filterType: 5 // 自适应滤镜
        });
        
        // 保存优化后的文件
        fs.writeFileSync(outputPath, optimizedData);
        
        const originalSize = data.length;
        const optimizedSize = optimizedData.length;
        const compressionRatio = ((originalSize - optimizedSize) / originalSize * 100).toFixed(2);
        
        console.log('优化完成!');
        console.log('原始大小:', originalSize, 'bytes');
        console.log('优化后大小:', optimizedSize, 'bytes');
        console.log('压缩比:', compressionRatio + '%');
        
    } catch (error) {
        console.error('处理失败:', error.message);
        process.exit(1);
    }
}

// 使用示例
if (require.main === module) {
    const inputPath = process.argv[2] || 'input.png';
    const outputPath = process.argv[3] || 'output.png';
    
    if (!fs.existsSync(inputPath)) {
        console.error('输入文件不存在:', inputPath);
        process.exit(1);
    }
    
    processPNGFile(inputPath, outputPath);
}

module.exports = { processPNGFile };
```

### 批量处理

```javascript
const { PNGSync, optimizePNG } = require('rust-png-js');
const fs = require('fs');
const path = require('path');
const { promisify } = require('util');

const readdir = promisify(fs.readdir);
const readFile = promisify(fs.readFile);
const writeFile = promisify(fs.writeFile);
const stat = promisify(fs.stat);

async function batchProcessPNG(inputDir, outputDir, options = {}) {
    const {
        maxConcurrency = 5,
        deflateLevel = 9,
        filterType = 5,
        verbose = false
    } = options;
    
    try {
        console.log('开始批量处理PNG文件...');
        console.log('输入目录:', inputDir);
        console.log('输出目录:', outputDir);
        
        // 检查输入目录
        if (!fs.existsSync(inputDir)) {
            throw new Error('输入目录不存在: ' + inputDir);
        }
        
        // 创建输出目录
        if (!fs.existsSync(outputDir)) {
            fs.mkdirSync(outputDir, { recursive: true });
        }
        
        // 获取所有PNG文件
        const files = await readdir(inputDir);
        const pngFiles = files.filter(file => path.extname(file).toLowerCase() === '.png');
        
        if (pngFiles.length === 0) {
            console.log('未找到PNG文件');
            return;
        }
        
        console.log(`找到 ${pngFiles.length} 个PNG文件`);
        
        // 并发处理
        const results = [];
        const semaphore = new Array(maxConcurrency).fill(null);
        
        for (let i = 0; i < pngFiles.length; i += maxConcurrency) {
            const batch = pngFiles.slice(i, i + maxConcurrency);
            const batchResults = await Promise.all(
                batch.map(async (file) => {
                    const inputPath = path.join(inputDir, file);
                    const outputPath = path.join(outputDir, file);
                    
                    try {
                        if (verbose) {
                            console.log(`处理文件: ${file}`);
                        }
                        
                        // 读取文件
                        const data = await readFile(inputPath);
                        
                        // 优化PNG
                        const optimizedData = await optimizePNG(data, {
                            deflateLevel,
                            filterType
                        });
                        
                        // 保存优化后的文件
                        await writeFile(outputPath, optimizedData);
                        
                        const originalSize = data.length;
                        const optimizedSize = optimizedData.length;
                        const compressionRatio = ((originalSize - optimizedSize) / originalSize * 100).toFixed(2);
                        
                        const result = {
                            file,
                            originalSize,
                            optimizedSize,
                            compressionRatio: parseFloat(compressionRatio),
                            success: true
                        };
                        
                        if (verbose) {
                            console.log(`  ${file}: ${compressionRatio}% 压缩`);
                        }
                        
                        return result;
                        
                    } catch (error) {
                        console.error(`处理文件 ${file} 失败:`, error.message);
                        return {
                            file,
                            error: error.message,
                            success: false
                        };
                    }
                })
            );
            
            results.push(...batchResults);
        }
        
        // 统计结果
        const successfulResults = results.filter(r => r.success);
        const failedResults = results.filter(r => !r.success);
        
        const totalOriginalSize = successfulResults.reduce((sum, r) => sum + r.originalSize, 0);
        const totalOptimizedSize = successfulResults.reduce((sum, r) => sum + r.optimizedSize, 0);
        const avgCompressionRatio = successfulResults.reduce((sum, r) => sum + r.compressionRatio, 0) / successfulResults.length;
        
        console.log('\n批量处理完成!');
        console.log(`成功处理: ${successfulResults.length}/${pngFiles.length} 个文件`);
        console.log(`失败: ${failedResults.length} 个文件`);
        console.log(`总原始大小: ${(totalOriginalSize / 1024 / 1024).toFixed(2)} MB`);
        console.log(`总优化后大小: ${(totalOptimizedSize / 1024 / 1024).toFixed(2)} MB`);
        console.log(`平均压缩比: ${avgCompressionRatio.toFixed(2)}%`);
        
        if (failedResults.length > 0) {
            console.log('\n失败的文件:');
            failedResults.forEach(r => {
                console.log(`  ${r.file}: ${r.error}`);
            });
        }
        
    } catch (error) {
        console.error('批量处理失败:', error.message);
        process.exit(1);
    }
}

// 使用示例
if (require.main === module) {
    const inputDir = process.argv[2] || './input';
    const outputDir = process.argv[3] || './output';
    
    const options = {
        maxConcurrency: 5,
        deflateLevel: 9,
        filterType: 5,
        verbose: true
    };
    
    batchProcessPNG(inputDir, outputDir, options);
}

module.exports = { batchProcessPNG };
```

## React示例

### 基本组件

```jsx
import React, { useState, useCallback } from 'react';
import { PNG, PNGSync, validatePNG, optimizePNG } from 'rust-png-js';

const PNGProcessor = () => {
    const [file, setFile] = useState(null);
    const [png, setPng] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const [imageUrl, setImageUrl] = useState(null);

    const handleFileChange = useCallback(async (event) => {
        const selectedFile = event.target.files[0];
        if (!selectedFile) return;

        setFile(selectedFile);
        setLoading(true);
        setError(null);

        try {
            // 读取文件数据
            const data = new Uint8Array(await selectedFile.arrayBuffer());
            
            // 验证PNG
            const isValid = await validatePNG(data);
            if (!isValid) {
                throw new Error('无效的PNG文件');
            }

            // 解析PNG
            const pngSync = new PNGSync();
            const pngData = pngSync.read(data);
            setPng(pngData);

            // 创建图像URL
            const blob = new Blob([data], { type: 'image/png' });
            const url = URL.createObjectURL(blob);
            setImageUrl(url);

        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    }, []);

    const handleOptimize = useCallback(async () => {
        if (!png) return;

        setLoading(true);
        try {
            // 优化PNG
            const optimizedData = await optimizePNG(png.pack(), {
                deflateLevel: 9,
                filterType: 5
            });

            // 下载优化后的文件
            const blob = new Blob([optimizedData], { type: 'image/png' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'optimized.png';
            a.click();
            URL.revokeObjectURL(url);

        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    }, [png]);

    return (
        <div style={{ padding: '20px' }}>
            <h1>PNG处理器</h1>
            
            <div style={{ marginBottom: '20px' }}>
                <input
                    type="file"
                    accept=".png"
                    onChange={handleFileChange}
                    disabled={loading}
                />
            </div>

            {loading && <p>处理中...</p>}
            
            {error && (
                <div style={{ color: 'red', marginBottom: '20px' }}>
                    错误: {error}
                </div>
            )}

            {imageUrl && (
                <div style={{ marginBottom: '20px' }}>
                    <img
                        src={imageUrl}
                        alt="PNG预览"
                        style={{ maxWidth: '100%', border: '1px solid #ddd' }}
                    />
                </div>
            )}

            {png && (
                <div style={{ marginBottom: '20px' }}>
                    <h3>图像信息</h3>
                    <div>
                        <p>尺寸: {png.width} x {png.height}</p>
                        <p>颜色类型: {png.getColorType()}</p>
                        <p>位深度: {png.getBitDepth()}</p>
                        <p>Alpha通道: {png.alpha ? '是' : '否'}</p>
                    </div>
                    
                    <button
                        onClick={handleOptimize}
                        disabled={loading}
                        style={{
                            background: '#007bff',
                            color: 'white',
                            border: 'none',
                            padding: '10px 20px',
                            borderRadius: '4px',
                            cursor: 'pointer'
                        }}
                    >
                        优化PNG
                    </button>
                </div>
            )}
        </div>
    );
};

export default PNGProcessor;
```

### 高级组件

```jsx
import React, { useState, useCallback, useRef } from 'react';
import { PNG, PNGSync, optimizePNG, benchmark } from 'rust-png-js';

const AdvancedPNGProcessor = () => {
    const [png, setPng] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const [stats, setStats] = useState(null);
    const canvasRef = useRef(null);

    const processPNG = useCallback(async (data) => {
        setLoading(true);
        setError(null);

        try {
            // 解析PNG
            const pngSync = new PNGSync();
            const pngData = pngSync.read(data);
            setPng(pngData);

            // 显示图像
            const canvas = canvasRef.current;
            if (canvas) {
                const ctx = canvas.getContext('2d');
                canvas.width = pngData.width;
                canvas.height = pngData.height;

                const imageData = ctx.createImageData(pngData.width, pngData.height);
                imageData.data.set(pngData.getRGBA8Array());
                ctx.putImageData(imageData, 0, 0);
            }

            // 运行性能测试
            const performanceStats = await benchmark(data, 10);
            setStats(performanceStats);

        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    }, []);

    const handleFileChange = useCallback(async (event) => {
        const file = event.target.files[0];
        if (!file) return;

        const data = new Uint8Array(await file.arrayBuffer());
        await processPNG(data);
    }, [processPNG]);

    const handleOptimize = useCallback(async () => {
        if (!png) return;

        setLoading(true);
        try {
            const optimizedData = await optimizePNG(png.pack(), {
                deflateLevel: 9,
                filterType: 5
            });

            // 下载优化后的文件
            const blob = new Blob([optimizedData], { type: 'image/png' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'optimized.png';
            a.click();
            URL.revokeObjectURL(url);

        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    }, [png]);

    return (
        <div style={{ padding: '20px' }}>
            <h1>高级PNG处理器</h1>
            
            <div style={{ marginBottom: '20px' }}>
                <input
                    type="file"
                    accept=".png"
                    onChange={handleFileChange}
                    disabled={loading}
                />
            </div>

            {loading && <p>处理中...</p>}
            
            {error && (
                <div style={{ color: 'red', marginBottom: '20px' }}>
                    错误: {error}
                </div>
            )}

            {png && (
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px' }}>
                    <div>
                        <h3>图像预览</h3>
                        <canvas
                            ref={canvasRef}
                            style={{ border: '1px solid #ddd', maxWidth: '100%' }}
                        />
                    </div>
                    
                    <div>
                        <h3>图像信息</h3>
                        <div style={{ background: '#f8f9fa', padding: '15px', borderRadius: '5px' }}>
                            <p>尺寸: {png.width} x {png.height}</p>
                            <p>颜色类型: {png.getColorType()}</p>
                            <p>位深度: {png.getBitDepth()}</p>
                            <p>压缩方法: {png.getCompressionMethod()}</p>
                            <p>滤镜方法: {png.getFilterMethod()}</p>
                            <p>交错方法: {png.getInterlaceMethod()}</p>
                            <p>Alpha通道: {png.alpha ? '是' : '否'}</p>
                            <p>Gamma值: {png.gamma}</p>
                        </div>
                        
                        {stats && (
                            <div style={{ marginTop: '20px' }}>
                                <h3>性能统计</h3>
                                <div style={{ background: '#e9ecef', padding: '15px', borderRadius: '5px' }}>
                                    <p>解析时间: {stats.parseTime.toFixed(2)} ms</p>
                                    <p>编码时间: {stats.encodeTime.toFixed(2)} ms</p>
                                    <p>内存使用: {(stats.memoryUsage / 1024 / 1024).toFixed(2)} MB</p>
                                    <p>压缩比: {stats.compressionRatio.toFixed(2)}%</p>
                                </div>
                            </div>
                        )}
                        
                        <button
                            onClick={handleOptimize}
                            disabled={loading}
                            style={{
                                background: '#28a745',
                                color: 'white',
                                border: 'none',
                                padding: '10px 20px',
                                borderRadius: '4px',
                                cursor: 'pointer',
                                marginTop: '20px'
                            }}
                        >
                            优化PNG
                        </button>
                    </div>
                </div>
            )}
        </div>
    );
};

export default AdvancedPNGProcessor;
```

## Vue示例

### 基本组件

```vue
<template>
  <div class="png-processor">
    <h1>PNG处理器</h1>
    
    <div class="file-input">
      <input
        type="file"
        accept=".png"
        @change="handleFileChange"
        :disabled="loading"
      />
    </div>

    <div v-if="loading" class="loading">
      处理中...
    </div>

    <div v-if="error" class="error">
      错误: {{ error }}
    </div>

    <div v-if="imageUrl" class="image-preview">
      <img :src="imageUrl" alt="PNG预览" />
    </div>

    <div v-if="png" class="image-info">
      <h3>图像信息</h3>
      <div class="info-content">
        <p>尺寸: {{ png.width }} x {{ png.height }}</p>
        <p>颜色类型: {{ png.getColorType() }}</p>
        <p>位深度: {{ png.getBitDepth() }}</p>
        <p>Alpha通道: {{ png.alpha ? '是' : '否' }}</p>
      </div>
      
      <button
        @click="handleOptimize"
        :disabled="loading"
        class="optimize-btn"
      >
        优化PNG
      </button>
    </div>
  </div>
</template>

<script>
import { PNG, PNGSync, validatePNG, optimizePNG } from 'rust-png-js';

export default {
  name: 'PNGProcessor',
  data() {
    return {
      file: null,
      png: null,
      loading: false,
      error: null,
      imageUrl: null
    };
  },
  methods: {
    async handleFileChange(event) {
      const selectedFile = event.target.files[0];
      if (!selectedFile) return;

      this.file = selectedFile;
      this.loading = true;
      this.error = null;

      try {
        // 读取文件数据
        const data = new Uint8Array(await selectedFile.arrayBuffer());
        
        // 验证PNG
        const isValid = await validatePNG(data);
        if (!isValid) {
          throw new Error('无效的PNG文件');
        }

        // 解析PNG
        const pngSync = new PNGSync();
        const pngData = pngSync.read(data);
        this.png = pngData;

        // 创建图像URL
        const blob = new Blob([data], { type: 'image/png' });
        const url = URL.createObjectURL(blob);
        this.imageUrl = url;

      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },

    async handleOptimize() {
      if (!this.png) return;

      this.loading = true;
      try {
        // 优化PNG
        const optimizedData = await optimizePNG(this.png.pack(), {
          deflateLevel: 9,
          filterType: 5
        });

        // 下载优化后的文件
        const blob = new Blob([optimizedData], { type: 'image/png' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'optimized.png';
        a.click();
        URL.revokeObjectURL(url);

      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    }
  }
};
</script>

<style scoped>
.png-processor {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.file-input {
  margin: 20px 0;
}

.loading {
  color: #007bff;
  margin: 20px 0;
}

.error {
  color: #dc3545;
  background: #f8d7da;
  padding: 15px;
  border-radius: 4px;
  margin: 20px 0;
}

.image-preview {
  text-align: center;
  margin: 20px 0;
}

.image-preview img {
  max-width: 100%;
  border: 1px solid #ddd;
}

.image-info {
  background: #f8f9fa;
  padding: 20px;
  border-radius: 5px;
  margin: 20px 0;
}

.info-content {
  margin: 15px 0;
}

.optimize-btn {
  background: #007bff;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
}

.optimize-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}
</style>
```

## Angular示例

### 基本组件

```typescript
import { Component } from '@angular/core';
import { PNG, PNGSync, validatePNG, optimizePNG } from 'rust-png-js';

@Component({
  selector: 'app-png-processor',
  template: `
    <div class="png-processor">
      <h1>PNG处理器</h1>
      
      <div class="file-input">
        <input
          type="file"
          accept=".png"
          (change)="handleFileChange($event)"
          [disabled]="loading"
        />
      </div>

      <div *ngIf="loading" class="loading">
        处理中...
      </div>

      <div *ngIf="error" class="error">
        错误: {{ error }}
      </div>

      <div *ngIf="imageUrl" class="image-preview">
        <img [src]="imageUrl" alt="PNG预览" />
      </div>

      <div *ngIf="png" class="image-info">
        <h3>图像信息</h3>
        <div class="info-content">
          <p>尺寸: {{ png.width }} x {{ png.height }}</p>
          <p>颜色类型: {{ png.getColorType() }}</p>
          <p>位深度: {{ png.getBitDepth() }}</p>
          <p>Alpha通道: {{ png.alpha ? '是' : '否' }}</p>
        </div>
        
        <button
          (click)="handleOptimize()"
          [disabled]="loading"
          class="optimize-btn"
        >
          优化PNG
        </button>
      </div>
    </div>
  `,
  styles: [`
    .png-processor {
      padding: 20px;
      max-width: 800px;
      margin: 0 auto;
    }

    .file-input {
      margin: 20px 0;
    }

    .loading {
      color: #007bff;
      margin: 20px 0;
    }

    .error {
      color: #dc3545;
      background: #f8d7da;
      padding: 15px;
      border-radius: 4px;
      margin: 20px 0;
    }

    .image-preview {
      text-align: center;
      margin: 20px 0;
    }

    .image-preview img {
      max-width: 100%;
      border: 1px solid #ddd;
    }

    .image-info {
      background: #f8f9fa;
      padding: 20px;
      border-radius: 5px;
      margin: 20px 0;
    }

    .info-content {
      margin: 15px 0;
    }

    .optimize-btn {
      background: #007bff;
      color: white;
      border: none;
      padding: 10px 20px;
      border-radius: 4px;
      cursor: pointer;
    }

    .optimize-btn:disabled {
      background: #ccc;
      cursor: not-allowed;
    }
  `]
})
export class PNGProcessorComponent {
  file: File | null = null;
  png: any = null;
  loading = false;
  error: string | null = null;
  imageUrl: string | null = null;

  async handleFileChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const selectedFile = target.files?.[0];
    if (!selectedFile) return;

    this.file = selectedFile;
    this.loading = true;
    this.error = null;

    try {
      // 读取文件数据
      const data = new Uint8Array(await selectedFile.arrayBuffer());
      
      // 验证PNG
      const isValid = await validatePNG(data);
      if (!isValid) {
        throw new Error('无效的PNG文件');
      }

      // 解析PNG
      const pngSync = new PNGSync();
      const pngData = pngSync.read(data);
      this.png = pngData;

      // 创建图像URL
      const blob = new Blob([data], { type: 'image/png' });
      const url = URL.createObjectURL(blob);
      this.imageUrl = url;

    } catch (err: any) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async handleOptimize() {
    if (!this.png) return;

    this.loading = true;
    try {
      // 优化PNG
      const optimizedData = await optimizePNG(this.png.pack(), {
        deflateLevel: 9,
        filterType: 5
      });

      // 下载优化后的文件
      const blob = new Blob([optimizedData], { type: 'image/png' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'optimized.png';
      a.click();
      URL.revokeObjectURL(url);

    } catch (err: any) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }
}
```

## Web Workers示例

### Worker文件

```javascript
// png-worker.js
import { PNG, PNGSync, validatePNG, optimizePNG, benchmark } from 'rust-png-js';

self.onmessage = async function(event) {
  const { type, data, options } = event.data;
  
  try {
    switch (type) {
      case 'validate':
        const isValid = await validatePNG(data);
        self.postMessage({ type: 'validate', result: isValid });
        break;
        
      case 'parse':
        const pngSync = new PNGSync();
        const png = pngSync.read(data);
        self.postMessage({ 
          type: 'parse', 
          result: {
            width: png.width,
            height: png.height,
            colorType: png.getColorType(),
            bitDepth: png.getBitDepth(),
            alpha: png.alpha
          }
        });
        break;
        
      case 'optimize':
        const optimizedData = await optimizePNG(data, options);
        self.postMessage({ type: 'optimize', result: optimizedData });
        break;
        
      case 'benchmark':
        const stats = await benchmark(data, options.iterations || 10);
        self.postMessage({ type: 'benchmark', result: stats });
        break;
        
      default:
        throw new Error('未知的操作类型: ' + type);
    }
  } catch (error) {
    self.postMessage({ 
      type: 'error', 
      error: error.message 
    });
  }
};
```

### 主线程使用

```javascript
// main.js
class PNGWorker {
  constructor() {
    this.worker = new Worker('./png-worker.js');
    this.callbacks = new Map();
    this.callbackId = 0;
    
    this.worker.onmessage = (event) => {
      const { type, result, error, callbackId } = event.data;
      
      if (callbackId && this.callbacks.has(callbackId)) {
        const callback = this.callbacks.get(callbackId);
        this.callbacks.delete(callbackId);
        
        if (error) {
          callback.reject(new Error(error));
        } else {
          callback.resolve(result);
        }
      }
    };
  }
  
  async validate(data) {
    return this.sendMessage('validate', data);
  }
  
  async parse(data) {
    return this.sendMessage('parse', data);
  }
  
  async optimize(data, options) {
    return this.sendMessage('optimize', data, options);
  }
  
  async benchmark(data, iterations) {
    return this.sendMessage('benchmark', data, { iterations });
  }
  
  sendMessage(type, data, options = {}) {
    return new Promise((resolve, reject) => {
      const callbackId = ++this.callbackId;
      this.callbacks.set(callbackId, { resolve, reject });
      
      this.worker.postMessage({
        type,
        data,
        options,
        callbackId
      });
    });
  }
  
  terminate() {
    this.worker.terminate();
  }
}

// 使用示例
const pngWorker = new PNGWorker();

async function processPNG(file) {
  try {
    const data = new Uint8Array(await file.arrayBuffer());
    
    // 验证PNG
    const isValid = await pngWorker.validate(data);
    if (!isValid) {
      throw new Error('无效的PNG文件');
    }
    
    // 解析PNG
    const pngInfo = await pngWorker.parse(data);
    console.log('PNG信息:', pngInfo);
    
    // 优化PNG
    const optimizedData = await pngWorker.optimize(data, {
      deflateLevel: 9,
      filterType: 5
    });
    
    // 性能测试
    const stats = await pngWorker.benchmark(data, 10);
    console.log('性能统计:', stats);
    
    return optimizedData;
    
  } catch (error) {
    console.error('处理失败:', error);
    throw error;
  }
}

// 清理
window.addEventListener('beforeunload', () => {
  pngWorker.terminate();
});
```

## 批量处理示例

### 批量优化工具

```javascript
const { PNGSync, optimizePNG } = require('rust-png-js');
const fs = require('fs');
const path = require('path');
const { promisify } = require('util');

class BatchPNGProcessor {
  constructor(options = {}) {
    this.options = {
      maxConcurrency: 5,
      deflateLevel: 9,
      filterType: 5,
      verbose: false,
      ...options
    };
  }
  
  async processDirectory(inputDir, outputDir) {
    console.log('开始批量处理PNG文件...');
    
    // 获取所有PNG文件
    const files = await this.getPNGFiles(inputDir);
    if (files.length === 0) {
      console.log('未找到PNG文件');
      return;
    }
    
    console.log(`找到 ${files.length} 个PNG文件`);
    
    // 创建输出目录
    await this.ensureDirectory(outputDir);
    
    // 并发处理
    const results = await this.processFiles(files, inputDir, outputDir);
    
    // 统计结果
    this.printResults(results);
    
    return results;
  }
  
  async getPNGFiles(dir) {
    const files = await promisify(fs.readdir)(dir);
    return files.filter(file => path.extname(file).toLowerCase() === '.png');
  }
  
  async ensureDirectory(dir) {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
  }
  
  async processFiles(files, inputDir, outputDir) {
    const results = [];
    
    for (let i = 0; i < files.length; i += this.options.maxConcurrency) {
      const batch = files.slice(i, i + this.options.maxConcurrency);
      const batchResults = await Promise.all(
        batch.map(file => this.processFile(file, inputDir, outputDir))
      );
      results.push(...batchResults);
    }
    
    return results;
  }
  
  async processFile(file, inputDir, outputDir) {
    const inputPath = path.join(inputDir, file);
    const outputPath = path.join(outputDir, file);
    
    try {
      if (this.options.verbose) {
        console.log(`处理文件: ${file}`);
      }
      
      // 读取文件
      const data = await promisify(fs.readFile)(inputPath);
      
      // 优化PNG
      const optimizedData = await optimizePNG(data, {
        deflateLevel: this.options.deflateLevel,
        filterType: this.options.filterType
      });
      
      // 保存优化后的文件
      await promisify(fs.writeFile)(outputPath, optimizedData);
      
      const originalSize = data.length;
      const optimizedSize = optimizedData.length;
      const compressionRatio = ((originalSize - optimizedSize) / originalSize * 100).toFixed(2);
      
      return {
        file,
        originalSize,
        optimizedSize,
        compressionRatio: parseFloat(compressionRatio),
        success: true
      };
      
    } catch (error) {
      console.error(`处理文件 ${file} 失败:`, error.message);
      return {
        file,
        error: error.message,
        success: false
      };
    }
  }
  
  printResults(results) {
    const successfulResults = results.filter(r => r.success);
    const failedResults = results.filter(r => !r.success);
    
    const totalOriginalSize = successfulResults.reduce((sum, r) => sum + r.originalSize, 0);
    const totalOptimizedSize = successfulResults.reduce((sum, r) => sum + r.optimizedSize, 0);
    const avgCompressionRatio = successfulResults.reduce((sum, r) => sum + r.compressionRatio, 0) / successfulResults.length;
    
    console.log('\n批量处理完成!');
    console.log(`成功处理: ${successfulResults.length}/${results.length} 个文件`);
    console.log(`失败: ${failedResults.length} 个文件`);
    console.log(`总原始大小: ${(totalOriginalSize / 1024 / 1024).toFixed(2)} MB`);
    console.log(`总优化后大小: ${(totalOptimizedSize / 1024 / 1024).toFixed(2)} MB`);
    console.log(`平均压缩比: ${avgCompressionRatio.toFixed(2)}%`);
    
    if (failedResults.length > 0) {
      console.log('\n失败的文件:');
      failedResults.forEach(r => {
        console.log(`  ${r.file}: ${r.error}`);
      });
    }
  }
}

// 使用示例
async function main() {
  const processor = new BatchPNGProcessor({
    maxConcurrency: 5,
    deflateLevel: 9,
    filterType: 5,
    verbose: true
  });
  
  try {
    await processor.processDirectory('./input', './output');
  } catch (error) {
    console.error('批量处理失败:', error);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}

module.exports = { BatchPNGProcessor };
```

## 高级功能示例

### 语义PNG处理

```javascript
import { SemanticPNG, SemanticPNGSync } from 'rust-png-js';

class SemanticPNGProcessor {
  constructor() {
    this.semanticPNG = new SemanticPNG();
    this.semanticPNGSync = new SemanticPNGSync();
  }
  
  async processSemanticPNG(data, metadata) {
    try {
      // 设置语义元数据
      this.semanticPNG.setSemanticMetadata(metadata);
      
      // 解析PNG
      this.semanticPNG.parse(data, (error, result) => {
        if (error) {
          throw error;
        }
        
        console.log('语义PNG解析成功');
        console.log('图像尺寸:', result.width, 'x', result.height);
        console.log('语义元数据:', result.getSemanticMetadata());
      });
      
      return this.semanticPNG;
      
    } catch (error) {
      console.error('语义PNG处理失败:', error);
      throw error;
    }
  }
  
  async processSemanticPNGSync(data, metadata) {
    try {
      // 同步处理
      const png = this.semanticPNGSync.readSemantic(data);
      
      // 设置语义元数据
      png.setSemanticMetadata(metadata);
      
      console.log('语义PNG同步处理成功');
      console.log('图像尺寸:', png.width, 'x', png.height);
      console.log('语义元数据:', png.getSemanticMetadata());
      
      return png;
      
    } catch (error) {
      console.error('语义PNG同步处理失败:', error);
      throw error;
    }
  }
}

// 使用示例
const processor = new SemanticPNGProcessor();

// 语义元数据
const metadata = {
  author: 'John Doe',
  description: 'My awesome image',
  tags: ['nature', 'landscape'],
  created: new Date(),
  location: {
    latitude: 40.7128,
    longitude: -74.0060
  },
  camera: {
    make: 'Canon',
    model: 'EOS R5',
    settings: {
      iso: 100,
      aperture: 'f/8',
      shutterSpeed: '1/125'
    }
  }
};

// 处理语义PNG
processor.processSemanticPNG(data, metadata)
  .then(png => {
    console.log('处理完成:', png);
  })
  .catch(error => {
    console.error('处理失败:', error);
  });
```

### 性能监控

```javascript
import { benchmark, getMemoryUsage, clearCache } from 'rust-png-js';

class PerformanceMonitor {
  constructor() {
    this.stats = [];
    this.startTime = 0;
  }
  
  start() {
    this.startTime = performance.now();
  }
  
  end() {
    return performance.now() - this.startTime;
  }
  
  async runBenchmark(data, iterations = 10) {
    console.log(`运行性能基准测试 (${iterations} 次迭代)...`);
    
    const stats = await benchmark(data, iterations);
    
    console.log('基准测试结果:');
    console.log(`  解析时间: ${stats.parseTime.toFixed(2)} ms`);
    console.log(`  编码时间: ${stats.encodeTime.toFixed(2)} ms`);
    console.log(`  内存使用: ${(stats.memoryUsage / 1024 / 1024).toFixed(2)} MB`);
    console.log(`  压缩比: ${stats.compressionRatio.toFixed(2)}%`);
    
    this.stats.push(stats);
    return stats;
  }
  
  async monitorMemory() {
    const memoryUsage = await getMemoryUsage();
    console.log(`当前内存使用: ${(memoryUsage / 1024 / 1024).toFixed(2)} MB`);
    
    if (memoryUsage > 100 * 1024 * 1024) { // 100MB
      console.log('内存使用过高，清理缓存...');
      await clearCache();
      console.log('缓存已清理');
    }
  }
  
  getAverageStats() {
    if (this.stats.length === 0) {
      return null;
    }
    
    const avgStats = {
      parseTime: this.stats.reduce((sum, s) => sum + s.parseTime, 0) / this.stats.length,
      encodeTime: this.stats.reduce((sum, s) => sum + s.encodeTime, 0) / this.stats.length,
      memoryUsage: this.stats.reduce((sum, s) => sum + s.memoryUsage, 0) / this.stats.length,
      compressionRatio: this.stats.reduce((sum, s) => sum + s.compressionRatio, 0) / this.stats.length
    };
    
    return avgStats;
  }
}

// 使用示例
const monitor = new PerformanceMonitor();

// 运行性能测试
monitor.runBenchmark(data, 20)
  .then(stats => {
    console.log('性能测试完成');
  })
  .catch(error => {
    console.error('性能测试失败:', error);
  });

// 监控内存
setInterval(async () => {
  await monitor.monitorMemory();
}, 30000); // 每30秒检查一次
```

---

## 总结

本文档提供了Rust PNG JS库的完整示例代码，涵盖了：

1. **浏览器示例**: 基本文件处理和图像编辑器
2. **Node.js示例**: 文件处理和批量处理
3. **框架示例**: React、Vue、Angular集成
4. **Web Workers示例**: 后台处理
5. **批量处理示例**: 高效批量处理
6. **高级功能示例**: 语义处理和性能监控

通过这些示例，您可以：

- 快速上手使用库的基本功能
- 了解如何在不同环境中集成
- 掌握高级功能和性能优化技巧
- 学习最佳实践和错误处理

如果您需要更多帮助，请查看项目的GitHub仓库或提交Issue。
