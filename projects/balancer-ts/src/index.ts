import { ChemicalParser } from './parser';
import { ChemicalBalancer } from './balancer';
import { ChemicalFormatter } from './formatter';

export { ChemicalParser } from './parser';
export { ChemicalBalancer } from './balancer';
export { ChemicalFormatter } from './formatter';
export { MatrixUtils } from './matrix';
export * from './types';

/**
 * 化学方程式配平器主类
 */
export class ChemicalEquationBalancer {
  private parser = new ChemicalParser();
  private balancer = new ChemicalBalancer();
  private formatter = new ChemicalFormatter();

  /**
   * 配平化学方程式并返回LaTeX格式
   */
  balanceToLatex(equation: string): string {
    try {
      const parsedEquation = this.parser.parseEquation(equation);
      const result = this.balancer.balance(parsedEquation);
      return this.formatter.formatToLatex(result);
    } catch (error) {
      return error instanceof Error ? error.message : 'Unknown error';
    }
  }

  /**
   * 配平化学方程式并返回纯文本格式
   */
  balanceToText(equation: string): string {
    try {
      const parsedEquation = this.parser.parseEquation(equation);
      const result = this.balancer.balance(parsedEquation);
      return this.formatter.formatToText(result);
    } catch (error) {
      return error instanceof Error ? error.message : 'Unknown error';
    }
  }

  /**
   * 配平化学方程式并返回HTML格式
   */
  balanceToHtml(equation: string): string {
    try {
      const parsedEquation = this.parser.parseEquation(equation);
      const result = this.balancer.balance(parsedEquation);
      return this.formatter.formatToHtml(result);
    } catch (error) {
      return error instanceof Error ? error.message : 'Unknown error';
    }
  }

  /**
   * 配平化学方程式并返回详细结果
   */
  balance(equation: string) {
    try {
      const parsedEquation = this.parser.parseEquation(equation);
      const result = this.balancer.balance(parsedEquation);
      
      return {
        success: result.success,
        latex: this.formatter.formatToLatex(result),
        text: this.formatter.formatToText(result),
        html: this.formatter.formatToHtml(result),
        equations: result.equations,
        error: result.error
      };
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      return {
        success: false,
        latex: errorMessage,
        text: errorMessage,
        html: errorMessage,
        equations: [],
        error: errorMessage
      };
    }
  }

  /**
   * 解析化学方程式
   */
  parse(equation: string) {
    return this.parser.parseEquation(equation);
  }

  /**
   * 解析化合物
   */
  parseCompound(formula: string) {
    return this.parser.parseCompound(formula);
  }
}