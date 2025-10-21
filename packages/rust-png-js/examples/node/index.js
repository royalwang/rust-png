/**
 * Rust PNG JS - Node.js示例
 */

const { PNG, PNGSync, validatePNG, getPNGInfo, optimizePNG, benchmark } = require('../dist/index.js');
const fs = require('fs');
const path = require('path');

async function main() {
    console.log('🚀 Rust PNG JS - Node.js示例');
    console.log('================================');

    try {
        // 检查WASM支持
        const { isWASMSupported } = require('../dist/index.js');
        console.log('WebAssembly支持:', isWASMSupported() ? '✅' : '❌');

        // 示例1: 基本PNG处理
        console.log('\n📁 示例1: 基本PNG处理');
        console.log('-------------------');

        // 创建一个简单的测试PNG数据
        const testPNGData = createTestPNGData();
        
        // 验证PNG数据
        const isValid = await validatePNG(testPNGData);
        console.log('PNG数据有效:', isValid);

        if (isValid) {
            // 获取PNG信息
            const info = await getPNGInfo(testPNGData);
            console.log('PNG信息:', info);

            // 同步解析
            const pngSync = new PNGSync();
            const png = pngSync.read(testPNGData);
            console.log('图像尺寸:', png.width, 'x', png.height);
            console.log('颜色类型:', png.getColorType());
            console.log('位深度:', png.getBitDepth());
        }

        // 示例2: 异步PNG处理
        console.log('\n📁 示例2: 异步PNG处理');
        console.log('-------------------');

        const png = new PNG();
        png.parse(testPNGData, (error, result) => {
            if (error) {
                console.error('解析失败:', error.message);
                return;
            }

            console.log('异步解析成功!');
            console.log('图像尺寸:', result.width, 'x', result.height);
            console.log('像素数据长度:', result.data.length);
        });

        // 示例3: PNG优化
        console.log('\n📁 示例3: PNG优化');
        console.log('-------------------');

        try {
            const optimizedData = await optimizePNG(testPNGData, {
                deflateLevel: 9,
                filterType: 5 // 自适应滤镜
            });

            const originalSize = testPNGData.length;
            const optimizedSize = optimizedData.length;
            const compressionRatio = ((originalSize - optimizedSize) / originalSize * 100).toFixed(2);

            console.log('原始大小:', originalSize, 'bytes');
            console.log('优化后大小:', optimizedSize, 'bytes');
            console.log('压缩比:', compressionRatio + '%');
        } catch (error) {
            console.log('优化失败:', error.message);
        }

        // 示例4: 性能基准测试
        console.log('\n📁 示例4: 性能基准测试');
        console.log('-------------------');

        try {
            const stats = await benchmark(testPNGData, 10);
            console.log('解析时间:', stats.parseTime.toFixed(2), 'ms');
            console.log('编码时间:', stats.encodeTime.toFixed(2), 'ms');
            console.log('内存使用:', (stats.memoryUsage / 1024 / 1024).toFixed(2), 'MB');
            console.log('压缩比:', stats.compressionRatio.toFixed(2) + '%');
        } catch (error) {
            console.log('基准测试失败:', error.message);
        }

        // 示例5: 文件处理
        console.log('\n📁 示例5: 文件处理');
        console.log('-------------------');

        const outputDir = path.join(__dirname, 'output');
        if (!fs.existsSync(outputDir)) {
            fs.mkdirSync(outputDir);
        }

        // 保存测试PNG文件
        const outputPath = path.join(outputDir, 'test.png');
        fs.writeFileSync(outputPath, testPNGData);
        console.log('测试PNG文件已保存:', outputPath);

        // 读取并处理文件
        const fileData = fs.readFileSync(outputPath);
        const filePNG = pngSync.read(fileData);
        console.log('文件PNG尺寸:', filePNG.width, 'x', filePNG.height);

        console.log('\n✅ 所有示例完成!');

    } catch (error) {
        console.error('❌ 示例执行失败:', error.message);
        console.error('错误详情:', error);
    }
}

/**
 * 创建测试PNG数据
 * 这是一个简单的1x1像素的PNG文件
 */
function createTestPNGData() {
    // PNG文件头
    const pngSignature = new Uint8Array([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    
    // IHDR chunk (图像头)
    const ihdrData = new Uint8Array([
        0x00, 0x00, 0x00, 0x0D, // 长度: 13
        0x49, 0x48, 0x44, 0x52, // 类型: IHDR
        0x00, 0x00, 0x00, 0x01, // 宽度: 1
        0x00, 0x00, 0x00, 0x01, // 高度: 1
        0x08,                   // 位深度: 8
        0x06,                   // 颜色类型: RGBA
        0x00,                   // 压缩方法: 0
        0x00,                   // 滤镜方法: 0
        0x00,                   // 交错方法: 0
        0x00, 0x00, 0x00, 0x00  // CRC (简化)
    ]);

    // IDAT chunk (图像数据)
    const idatData = new Uint8Array([
        0x00, 0x00, 0x00, 0x0C, // 长度: 12
        0x49, 0x44, 0x41, 0x54, // 类型: IDAT
        0x78, 0x9C, 0x63, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, // 压缩数据
        0x00, 0x00, 0x00, 0x00  // CRC (简化)
    ]);

    // IEND chunk (图像结束)
    const iendData = new Uint8Array([
        0x00, 0x00, 0x00, 0x00, // 长度: 0
        0x49, 0x45, 0x4E, 0x44, // 类型: IEND
        0xAE, 0x42, 0x60, 0x82  // CRC
    ]);

    // 组合所有数据
    const result = new Uint8Array(
        pngSignature.length + 
        ihdrData.length + 
        idatData.length + 
        iendData.length
    );

    let offset = 0;
    result.set(pngSignature, offset);
    offset += pngSignature.length;
    
    result.set(ihdrData, offset);
    offset += ihdrData.length;
    
    result.set(idatData, offset);
    offset += idatData.length;
    
    result.set(iendData, offset);

    return result;
}

// 运行示例
if (require.main === module) {
    main().catch(console.error);
}

module.exports = { main, createTestPNGData };
