import { ChemicalEquation, ChemicalCompound, ChemicalNode, BalanceResult } from './types';

/**
 * 化学方程式格式化器
 */
export class ChemicalFormatter {
  /**
   * 格式化配平结果为LaTeX
   */
  formatToLatex(result: BalanceResult): string {
    if (!result.success || result.equations.length === 0) {
      return result.error || 'No solution';
    }

    if (result.equations.length === 1) {
      return this.formatEquationToLatex(result.equations[0]);
    }

    // 多个解，使用aligned环境
    const equations = result.equations.map(eq => this.formatEquationToLatex(eq));
    return `\\begin{aligned}\n${equations.join(' \\\\ \n')}\n\\end{aligned}`;
  }

  /**
   * 格式化单个方程式为LaTeX
   */
  formatEquationToLatex(equation: ChemicalEquation): string {
    const reactants = equation.reactants.map(compound => this.formatCompoundToLatex(compound));
    const products = equation.products.map(compound => this.formatCompoundToLatex(compound));
    
    return `${reactants.join(' + ')} \\rightarrow ${products.join(' + ')}`;
  }

  /**
   * 格式化化合物为LaTeX
   */
  formatCompoundToLatex(compound: ChemicalCompound): string {
    const coefficient = compound.coefficient > 1 ? compound.coefficient.toString() : '';
    const formula = compound.nodes.map(node => this.formatNodeToLatex(node)).join('');
    
    return coefficient + formula;
  }

  /**
   * 格式化节点为LaTeX
   */
  formatNodeToLatex(node: ChemicalNode): string {
    switch (node.type) {
      case 'element':
        const count = node.count > 1 ? `_{${node.count}}` : '';
        return `\\text{${node.symbol}}${count}`;
        
      case 'group':
        const groupContent = node.elements.map(el => this.formatNodeToLatex(el)).join('');
        const groupCount = node.count > 1 ? `_{${node.count}}` : '';
        return `(${groupContent})${groupCount}`;
        
      case 'ion':
        const ionContent = node.elements.map(el => this.formatNodeToLatex(el)).join('');
        const charge = this.formatCharge(node.charge);
        const ionCount = node.count > 1 ? `_{${node.count}}` : '';
        return `[${ionContent}]^{${charge}}${ionCount}`;
        
      default:
        return '';
    }
  }

  /**
   * 格式化电荷
   */
  private formatCharge(charge: number): string {
    if (charge === 0) return '';
    
    const absCharge = Math.abs(charge);
    const sign = charge > 0 ? '+' : '-';
    
    if (absCharge === 1) {
      return sign;
    }
    
    return `${absCharge}${sign}`;
  }

  /**
   * 格式化为纯文本
   */
  formatToText(result: BalanceResult): string {
    if (!result.success || result.equations.length === 0) {
      return result.error || 'No solution';
    }

    return result.equations.map(eq => this.formatEquationToText(eq)).join('\n');
  }

  /**
   * 格式化单个方程式为纯文本
   */
  formatEquationToText(equation: ChemicalEquation): string {
    const reactants = equation.reactants.map(compound => this.formatCompoundToText(compound));
    const products = equation.products.map(compound => this.formatCompoundToText(compound));
    
    return `${reactants.join(' + ')} → ${products.join(' + ')}`;
  }

  /**
   * 格式化化合物为纯文本
   */
  formatCompoundToText(compound: ChemicalCompound): string {
    const coefficient = compound.coefficient > 1 ? compound.coefficient.toString() : '';
    const formula = compound.nodes.map(node => this.formatNodeToText(node)).join('');
    
    return coefficient + formula;
  }

  /**
   * 格式化节点为纯文本
   */
  formatNodeToText(node: ChemicalNode): string {
    switch (node.type) {
      case 'element':
        const count = node.count > 1 ? node.count.toString() : '';
        return node.symbol + count;
        
      case 'group':
        const groupContent = node.elements.map(el => this.formatNodeToText(el)).join('');
        const groupCount = node.count > 1 ? node.count.toString() : '';
        return `(${groupContent})${groupCount}`;
        
      case 'ion':
        const ionContent = node.elements.map(el => this.formatNodeToText(el)).join('');
        const charge = this.formatChargeText(node.charge);
        const ionCount = node.count > 1 ? node.count.toString() : '';
        return `[${ionContent}]${charge}${ionCount}`;
        
      default:
        return '';
    }
  }

  /**
   * 格式化电荷为纯文本
   */
  private formatChargeText(charge: number): string {
    if (charge === 0) return '';
    
    const absCharge = Math.abs(charge);
    const sign = charge > 0 ? '+' : '-';
    
    if (absCharge === 1) {
      return sign;
    }
    
    return `${absCharge}${sign}`;
  }

  /**
   * 格式化为HTML
   */
  formatToHtml(result: BalanceResult): string {
    if (!result.success || result.equations.length === 0) {
      return result.error || 'No solution';
    }

    return result.equations.map(eq => this.formatEquationToHtml(eq)).join('<br>');
  }

  /**
   * 格式化单个方程式为HTML
   */
  formatEquationToHtml(equation: ChemicalEquation): string {
    const reactants = equation.reactants.map(compound => this.formatCompoundToHtml(compound));
    const products = equation.products.map(compound => this.formatCompoundToHtml(compound));
    
    return `${reactants.join(' + ')} → ${products.join(' + ')}`;
  }

  /**
   * 格式化化合物为HTML
   */
  formatCompoundToHtml(compound: ChemicalCompound): string {
    const coefficient = compound.coefficient > 1 ? compound.coefficient.toString() : '';
    const formula = compound.nodes.map(node => this.formatNodeToHtml(node)).join('');
    
    return coefficient + formula;
  }

  /**
   * 格式化节点为HTML
   */
  formatNodeToHtml(node: ChemicalNode): string {
    switch (node.type) {
      case 'element':
        const count = node.count > 1 ? `<sub>${node.count}</sub>` : '';
        return node.symbol + count;
        
      case 'group':
        const groupContent = node.elements.map(el => this.formatNodeToHtml(el)).join('');
        const groupCount = node.count > 1 ? `<sub>${node.count}</sub>` : '';
        return `(${groupContent})${groupCount}`;
        
      case 'ion':
        const ionContent = node.elements.map(el => this.formatNodeToHtml(el)).join('');
        const charge = this.formatChargeHtml(node.charge);
        const ionCount = node.count > 1 ? `<sub>${node.count}</sub>` : '';
        return `[${ionContent}]${charge}${ionCount}`;
        
      default:
        return '';
    }
  }

  /**
   * 格式化电荷为HTML
   */
  private formatChargeHtml(charge: number): string {
    if (charge === 0) return '';
    
    const absCharge = Math.abs(charge);
    const sign = charge > 0 ? '+' : '−';
    
    if (absCharge === 1) {
      return `<sup>${sign}</sup>`;
    }
    
    return `<sup>${absCharge}${sign}</sup>`;
  }
}