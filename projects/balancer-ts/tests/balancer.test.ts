import { ChemicalEquationBalancer } from '../src/index';

describe('ChemicalEquationBalancer', () => {
  let balancer: ChemicalEquationBalancer;

  beforeEach(() => {
    balancer = new ChemicalEquationBalancer();
  });

  describe('基本配平测试', () => {
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

  describe('离子配平测试', () => {
    test('简单离子反应', () => {
      const result = balancer.balance('[Ag]+ + [Cl]- = AgCl');
      expect(result.success).toBe(true);
    });

    test('复杂离子反应', () => {
      const result = balancer.balance('[MnO4]- + [H]+ + [Cl]- = [Mn]2+ + H2O + Cl2');
      expect(result.success).toBe(true);
    });
  });

  describe('虚拟元素测试', () => {
    test('含有Ph基团', () => {
      const result = balancer.balance('PhCH3 + O2 = CO2 + H2O');
      expect(result.success).toBe(true);
    });

    test('含有Et基团', () => {
      const result = balancer.balance('EtOH + O2 = CO2 + H2O');
      expect(result.success).toBe(true);
    });
  });

  describe('组合结构测试', () => {
    test('含有括号的化合物', () => {
      const result = balancer.balance('Ca(OH)2 + HCl = CaCl2 + H2O');
      expect(result.success).toBe(true);
    });

    test('复杂的括号结构', () => {
      const result = balancer.balance('Al2(SO4)3 + Ca(OH)2 = Al(OH)3 + CaSO4');
      expect(result.success).toBe(true);
    });
  });

  describe('格式化测试', () => {
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

  describe('错误处理测试', () => {
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

  describe('解析器测试', () => {
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

  describe('边界情况测试', () => {
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
});