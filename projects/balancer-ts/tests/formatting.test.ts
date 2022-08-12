import {ChemicalEquationBalancer} from '../src/index';

describe('格式化测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('LaTeX格式输出', () => {
        const result = balancer.balance('H2 + O2 = H2O');
        expect(result.latex).toContain('\\rightarrow');
        expect(result.latex).toContain('\\text{H}');
    });

    test('HTML格式输出', () => {
        const result = balancer.balance('H2 + O2 = H2O');
        expect(result.html).toContain('H<sub>2</sub>');
    });

    test('纯文本格式输出', () => {
        const result = balancer.balance('H2 + O2 = H2O');
        expect(result.text).toContain('→');
    });
});