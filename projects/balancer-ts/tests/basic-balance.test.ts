import {ChemicalEquationBalancer} from '../src/index';

describe('基本配平测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('简单的燃烧反应', () => {
        const result = balancer.balance('CH4 + O2 = CO2 + H2O');
        expect(result.success).toBe(true);
        expect(result.text).toContain('CH4 + 2O2 → CO2 + 2H2O');
    });

    test('复杂的氧化还原反应', () => {
        const result = balancer.balance('KMnO4 + HCl = KCl + MnCl2 + H2O + Cl2');
        expect(result.success).toBe(true);
    });

    test('含有系数的反应', () => {
        const result = balancer.balance('2H2 + O2 = 2H2O');
        expect(result.success).toBe(true);
    });
});