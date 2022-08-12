import {ChemicalEquationBalancer} from '../src/index';

describe('解析器测试', () => {
    let balancer: ChemicalEquationBalancer;

    beforeEach(() => {
        balancer = new ChemicalEquationBalancer();
    });

    test('解析简单化合物', () => {
        const compound = balancer.parseCompound('H2O');
        expect(compound.nodes).toHaveLength(2);
        expect(compound.coefficient).toBe(1);
    });

    test('解析含系数的化合物', () => {
        const compound = balancer.parseCompound('2H2SO4');
        expect(compound.coefficient).toBe(2);
    });

    test('解析含括号的化合物', () => {
        const compound = balancer.parseCompound('Ca(OH)2');
        expect(compound.nodes).toHaveLength(2);
    });

    test('解析离子', () => {
        const compound = balancer.parseCompound('[SO4]2-');
        expect(compound.nodes).toHaveLength(1);
        expect(compound.nodes[0].type).toBe('ion');
    });
});