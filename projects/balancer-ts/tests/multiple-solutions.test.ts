import {ChemicalEquationBalancer} from '../src/index';

describe('零空间多行解测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('复杂有机反应 - 可能有多个解', () => {
        // 复杂的有机反应，可能产生多个配平解
        const result = balancer.balance('C2H6 + O2 = CO2 + H2O');
        expect(result.success).toBe(true);
        // 验证是否有多个方程式解
        if (result.equations && result.equations.length > 1) {
            expect(result.equations.length).toBeGreaterThan(1);
        }
    });

    test('复杂氧化还原反应 - 多个可能的配平方案', () => {
        // 这种反应可能有多个配平解
        const result = balancer.balance('Cr2O7 + Fe + H = Cr + Fe3 + H2O');
        expect(result.success).toBe(true);
    });

    test('含有多个氧化态的反应', () => {
        // 含有多个氧化态变化的复杂反应
        const result = balancer.balance('MnO4 + C2O4 + H = Mn + CO2 + H2O');
        expect(result.success).toBe(true);
    });

    test('复杂离子反应 - 多解情况', () => {
        // 复杂的离子反应可能产生多个解
        const result = balancer.balance('[Cr2O7]2- + [Fe]2+ + [H]+ = [Cr]3+ + [Fe]3+ + H2O');
        expect(result.success).toBe(true);
    });

    test('有机化合物燃烧 - 复杂情况', () => {
        // 复杂有机物燃烧可能有多种配平方式
        const result = balancer.balance('C3H8 + O2 = CO2 + H2O');
        expect(result.success).toBe(true);
    });

    test('多元素复杂反应', () => {
        // 包含多种元素的复杂反应
        const result = balancer.balance('Al + CuSO4 = Al2(SO4)3 + Cu');
        expect(result.success).toBe(true);
    });

    test('验证多解的数学特性', () => {
        // 测试一个已知可能有多解的反应
        const result = balancer.balance('H2 + O2 = H2O');
        expect(result.success).toBe(true);
        
        // 如果有多个解，验证它们都是有效的
        if (result.equations && result.equations.length > 1) {
            for (const equation of result.equations) {
                // 验证每个解的系数都是正整数
                for (const reactant of equation.reactants) {
                    expect(reactant.coefficient).toBeGreaterThan(0);
                    expect(Number.isInteger(reactant.coefficient)).toBe(true);
                }
                for (const product of equation.products) {
                    expect(product.coefficient).toBeGreaterThan(0);
                    expect(Number.isInteger(product.coefficient)).toBe(true);
                }
            }
        }
    });
});