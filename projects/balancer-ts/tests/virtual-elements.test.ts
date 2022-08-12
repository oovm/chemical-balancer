import {ChemicalEquationBalancer} from '../src/index';

describe('虚拟元素测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('含有Ph基团', () => {
        const result = balancer.balance('PhCH3 + O2 = CO2 + H2O');
        expect(result.success).toBe(true);
    });

    test('含有Et基团', () => {
        const result = balancer.balance('EtOH + O2 = CO2 + H2O');
        expect(result.success).toBe(true);
    });
});