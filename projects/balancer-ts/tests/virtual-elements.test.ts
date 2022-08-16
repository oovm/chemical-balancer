import {ChemicalEquationBalancer} from '../src/index';

describe('虚拟元素测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('含有Ph基团', () => {
        const result = balancer.balance('PhCH3 + O2 = CO2 + H2O');
        expect(result.success).toBe(true);
        except(result.text).toContain('PhCH3 + 2O2 → CO2 + 2H2O');
    });

    test('含有Et基团', () => {
        const result = balancer.balance('EtOH + O2 = CO2 + H2O');
        expect(result.success).toBe(true);
        expect(result.text).toContain('EtOH + 2O2 → CO2 + 2H2O');
    });
});