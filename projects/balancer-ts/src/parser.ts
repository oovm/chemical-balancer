import { ChemicalElement, ChemicalGroup, ChemicalIon, ChemicalNode, ChemicalCompound, ChemicalEquation } from './types';

/**
 * 化学式解析器
 */
export class ChemicalParser {
  private input: string = '';
  private position: number = 0;

  /**
   * 解析化学方程式
   */
  parseEquation(equation: string): ChemicalEquation {
    const parts = equation.split(/\s*[=→]\s*/);
    if (parts.length !== 2) {
      throw new Error('Invalid equation format');
    }

    const reactants = this.parseCompounds(parts[0]);
    const products = this.parseCompounds(parts[1]);

    return { reactants, products };
  }

  /**
   * 解析多个化合物（用+分隔）
   */
  private parseCompounds(compoundsStr: string): ChemicalCompound[] {
    return compoundsStr.split(/\s*\+\s*/).map(compound => this.parseCompound(compound.trim()));
  }

  /**
   * 解析单个化合物
   */
  parseCompound(formula: string): ChemicalCompound {
    this.input = formula.trim();
    this.position = 0;

    // 解析系数
    const coefficient = this.parseCoefficient();
    
    // 解析化学式主体
    const nodes: ChemicalNode[] = [];
    while (this.position < this.input.length) {
      nodes.push(this.parseNode());
    }

    return { nodes, coefficient };
  }

  /**
   * 解析系数
   */
  private parseCoefficient(): number {
    let coefficient = '';
    while (this.position < this.input.length && /\d/.test(this.input[this.position])) {
      coefficient += this.input[this.position];
      this.position++;
    }
    return coefficient ? parseInt(coefficient) : 1;
  }

  /**
   * 解析节点（元素、组或离子）
   */
  private parseNode(): ChemicalNode {
    if (this.peek() === '(') {
      return this.parseGroup();
    } else if (this.peek() === '[') {
      return this.parseIon();
    } else {
      return this.parseElement();
    }
  }

  /**
   * 解析元素
   */
  private parseElement(): ChemicalElement {
    const symbol = this.parseElementSymbol();
    const count = this.parseNumber();
    return { type: 'element', symbol, count };
  }

  /**
   * 解析元素符号（支持虚拟元素如Ph）
   */
  private parseElementSymbol(): string {
    if (this.position >= this.input.length || !/[A-Z]/.test(this.input[this.position])) {
      throw new Error(`Expected element symbol at position ${this.position}`);
    }

    let symbol = this.input[this.position];
    this.position++;

    // 读取小写字母
    while (this.position < this.input.length && /[a-z]/.test(this.input[this.position])) {
      symbol += this.input[this.position];
      this.position++;
    }

    return symbol;
  }

  /**
   * 解析组 ()
   */
  private parseGroup(): ChemicalGroup {
    this.expect('(');
    const elements: ChemicalNode[] = [];
    
    while (this.peek() !== ')') {
      elements.push(this.parseNode());
    }
    
    this.expect(')');
    const count = this.parseNumber();
    
    return { type: 'group', elements, count };
  }

  /**
   * 解析离子 []
   */
  private parseIon(): ChemicalIon {
    this.expect('[');
    const elements: ChemicalNode[] = [];
    
    while (this.peek() !== ']') {
      elements.push(this.parseNode());
    }
    
    this.expect(']');
    
    // 解析电荷
    const charge = this.parseCharge();
    const count = this.parseNumber();
    
    return { type: 'ion', elements, charge, count };
  }

  /**
   * 解析电荷
   */
  private parseCharge(): number {
    if (this.position >= this.input.length) {
      return 0;
    }

    const sign = this.peek();
    if (sign !== '+' && sign !== '-') {
      return 0;
    }

    this.position++;
    const chargeNum = this.parseNumber();
    return sign === '+' ? chargeNum : -chargeNum;
  }

  /**
   * 解析数字
   */
  private parseNumber(): number {
    let numStr = '';
    while (this.position < this.input.length && /\d/.test(this.input[this.position])) {
      numStr += this.input[this.position];
      this.position++;
    }
    return numStr ? parseInt(numStr) : 1;
  }

  /**
   * 查看当前字符
   */
  private peek(): string {
    return this.position < this.input.length ? this.input[this.position] : '';
  }

  /**
   * 期望特定字符
   */
  private expect(char: string): void {
    if (this.peek() !== char) {
      throw new Error(`Expected '${char}' at position ${this.position}, got '${this.peek()}'`);
    }
    this.position++;
  }
}