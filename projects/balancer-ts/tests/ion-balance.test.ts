import {ChemicalEquationBalancer} from '../src/index';

describe('离子配平测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('简单离子反应', () => {
        const result = balancer.balance('[Ag]+ + [Cl]- = AgCl');
        expect(result.success).toBe(true);
    });

    test('复杂离子反应', () => {
        const result = balancer.balance('[MnO4]- + [H]+ + [Cl]- = [Mn]2+ + H2O + Cl2');
        expect(result.success).toBe(true);
    });
});