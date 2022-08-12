import {ChemicalEquationBalancer} from '../src/index';

describe('组合结构测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('含有括号的化合物', () => {
        const result = balancer.balance('Ca(OH)2 + HCl = CaCl2 + H2O');
        expect(result.success).toBe(true);
    });

    test('复杂的括号结构', () => {
        const result = balancer.balance('Al2(SO4)3 + Ca(OH)2 = Al(OH)3 + CaSO4');
        expect(result.success).toBe(true);
    });
});