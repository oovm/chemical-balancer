import {ChemicalEquationBalancer} from '../src/index';

describe('错误处理测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('无效的方程式格式', () => {
        const result = balancer.balance('H2 + O2');
        expect(result.success).toBe(false);
        expect(result.error).toBeDefined();
    });

    test('空字符串', () => {
        const result = balancer.balance('');
        expect(result.success).toBe(false);
    });

    test('无效的化学式', () => {
        const result = balancer.balance('123 + 456 = 789');
        expect(result.success).toBe(false);
    });
});