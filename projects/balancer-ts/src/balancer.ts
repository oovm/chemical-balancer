import { ChemicalEquation, ChemicalCompound, ChemicalNode, ElementCount, BalanceResult, Matrix } from './types';
import { MatrixUtils } from './matrix';

/**
 * 化学方程式配平器
 */
export class ChemicalBalancer {
  /**
   * 配平化学方程式
   */
  balance(equation: ChemicalEquation): BalanceResult {
    try {
      // 获取所有化合物
      const compounds = [...equation.reactants, ...equation.products];
      
      // 提取所有元素
      const elements = this.extractElements(compounds);
      
      if (elements.length === 0) {
        return { success: false, equations: [], error: 'No elements found' };
      }

      // 构建系数矩阵
      const matrix = this.buildCoefficientMatrix(compounds, elements);
      
      // 求解零空间
      const nullSpace = MatrixUtils.nullSpace(matrix);
      
      if (nullSpace.length === 0) {
        return { success: false, equations: [], error: 'No solution found' };
      }

      // 生成配平结果
      const balancedEquations = this.generateBalancedEquations(equation, nullSpace);
      
      return {
        success: true,
        equations: balancedEquations
      };
    } catch (error) {
      return {
        success: false,
        equations: [],
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  /**
   * 提取所有元素（包括电荷）
   */
  private extractElements(compounds: ChemicalCompound[]): string[] {
    const elementSet = new Set<string>();
    
    for (const compound of compounds) {
      const elements = this.getCompoundElements(compound);
      for (const element of Object.keys(elements)) {
        elementSet.add(element);
      }
    }
    
    return Array.from(elementSet).sort();
  }

  /**
   * 获取化合物中的元素计数
   */
  private getCompoundElements(compound: ChemicalCompound): ElementCount {
    const elements: ElementCount = {};
    
    for (const node of compound.nodes) {
      const nodeElements = this.getNodeElements(node);
      for (const [element, count] of Object.entries(nodeElements)) {
        elements[element] = (elements[element] || 0) + count;
      }
    }
    
    return elements;
  }

  /**
   * 获取节点中的元素计数
   */
  private getNodeElements(node: ChemicalNode): ElementCount {
    const elements: ElementCount = {};
    
    switch (node.type) {
      case 'element':
        elements[node.symbol] = node.count;
        break;
        
      case 'group':
        for (const childNode of node.elements) {
          const childElements = this.getNodeElements(childNode);
          for (const [element, count] of Object.entries(childElements)) {
            elements[element] = (elements[element] || 0) + count * node.count;
          }
        }
        break;
        
      case 'ion':
        // 处理离子中的元素
        for (const childNode of node.elements) {
          const childElements = this.getNodeElements(childNode);
          for (const [element, count] of Object.entries(childElements)) {
            elements[element] = (elements[element] || 0) + count * node.count;
          }
        }
        // 添加电荷作为特殊"元素"
        if (node.charge !== 0) {
          elements['CHARGE'] = (elements['CHARGE'] || 0) + node.charge * node.count;
        }
        break;
    }
    
    return elements;
  }

  /**
   * 构建系数矩阵
   */
  private buildCoefficientMatrix(compounds: ChemicalCompound[], elements: string[]): Matrix {
    const matrix: Matrix = [];
    
    // 为每个元素创建一行
    for (const element of elements) {
      const row: number[] = [];
      
      for (const compound of compounds) {
        const compoundElements = this.getCompoundElements(compound);
        row.push(compoundElements[element] || 0);
      }
      
      matrix.push(row);
    }
    
    // 调整反应物和生成物的符号
    const reactantCount = compounds.length - this.getProductCount(compounds.length);
    
    for (let i = 0; i < matrix.length; i++) {
      for (let j = reactantCount; j < matrix[i].length; j++) {
        matrix[i][j] = -matrix[i][j];
      }
    }
    
    return matrix;
  }

  /**
   * 获取生成物数量（假设反应物和生成物数量相等或接近）
   */
  private getProductCount(totalCount: number): number {
    return Math.floor(totalCount / 2);
  }

  /**
   * 生成配平后的方程式
   */
  private generateBalancedEquations(originalEquation: ChemicalEquation, nullSpace: Matrix): ChemicalEquation[] {
    const equations: ChemicalEquation[] = [];
    
    for (const solution of nullSpace) {
      // 确保系数为正整数
      const coefficients = this.normalizeCoefficients(solution);
      
      // 应用系数到原方程式
      const balancedEquation = this.applyCoefficients(originalEquation, coefficients);
      equations.push(balancedEquation);
    }
    
    return equations;
  }

  /**
   * 标准化系数（确保为正整数）
   */
  private normalizeCoefficients(solution: number[]): number[] {
    // 找到最小的正数
    const positiveValues = solution.filter(x => x > 0);
    if (positiveValues.length === 0) {
      return solution.map(() => 1);
    }
    
    const minPositive = Math.min(...positiveValues);
    
    // 归一化
    let normalized = solution.map(x => Math.abs(x / minPositive));
    
    // 转换为整数
    const gcd = this.findGCD(normalized.filter(x => x > 0));
    if (gcd > 0) {
      normalized = normalized.map(x => Math.round(x / gcd));
    }
    
    // 确保最小系数为1
    const minCoeff = Math.min(...normalized.filter(x => x > 0));
    if (minCoeff > 1) {
      normalized = normalized.map(x => Math.round(x / minCoeff));
    }
    
    return normalized.map(x => Math.max(1, Math.round(x)));
  }

  /**
   * 计算最大公约数
   */
  private findGCD(numbers: number[]): number {
    if (numbers.length === 0) return 1;
    
    let result = Math.abs(Math.round(numbers[0]));
    for (let i = 1; i < numbers.length; i++) {
      result = this.gcd(result, Math.abs(Math.round(numbers[i])));
    }
    return result;
  }

  /**
   * 计算两个数的最大公约数
   */
  private gcd(a: number, b: number): number {
    while (b !== 0) {
      const temp = b;
      b = a % b;
      a = temp;
    }
    return a;
  }

  /**
   * 应用系数到方程式
   */
  private applyCoefficients(equation: ChemicalEquation, coefficients: number[]): ChemicalEquation {
    const allCompounds = [...equation.reactants, ...equation.products];
    
    if (coefficients.length !== allCompounds.length) {
      throw new Error('Coefficient count mismatch');
    }
    
    const reactantCount = equation.reactants.length;
    
    const newReactants = equation.reactants.map((compound, index) => ({
      ...compound,
      coefficient: coefficients[index]
    }));
    
    const newProducts = equation.products.map((compound, index) => ({
      ...compound,
      coefficient: coefficients[reactantCount + index]
    }));
    
    return {
      reactants: newReactants,
      products: newProducts
    };
  }
}