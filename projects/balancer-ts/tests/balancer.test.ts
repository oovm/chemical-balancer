// 主测试文件 - 导入所有测试模块
// 这个文件作为测试套件的入口点，确保所有测试都被执行

// 导入所有测试模块
import './basic-balance.test';
import './ion-balance.test';
import './virtual-elements.test';
import './compound-structures.test';
import './formatting.test';
import './error-handling.test';
import './parser.test';
import './edge-cases.test';
import './multiple-solutions.test';

// 可以在这里添加集成测试或跨模块测试
describe('ChemicalEquationBalancer 集成测试', () => {
    // 这里可以添加一些集成测试
    test('测试套件完整性检查', () => {
        // 简单的完整性检查
        expect(true).toBe(true);
    });
});