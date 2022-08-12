import {ChemicalEquationBalancer} from '../src/index';

describe('边界情况测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('单一元素反应', () => {
        const result = balancer.balance('H = H');
        expect(result.success).toBe(true);
    });

    test('已经配平的方程式', () => {
        const result = balancer.balance('2H2 + O2 = 2H2O');
        expect(result.success).toBe(true);
    });

    test('大分子化合物', () => {
        const result = balancer.balance('C6H12O6 + O2 = CO2 + H2O');
        expect(result.success).toBe(true);
    });
});