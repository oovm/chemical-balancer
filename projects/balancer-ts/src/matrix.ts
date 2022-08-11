import { Matrix, MatrixRow } from './types';
import Fraction from 'fraction.js';

/**
 * 矩阵运算工具类
 */
export class MatrixUtils {
  /**
   * 创建零矩阵
   */
  static zeros(rows: number, cols: number): Matrix {
    return Array(rows).fill(0).map(() => Array(cols).fill(0));
  }

  /**
   * 矩阵转置
   */
  static transpose(matrix: Matrix): Matrix {
    if (matrix.length === 0) return [];
    const rows = matrix.length;
    const cols = matrix[0].length;
    const result = this.zeros(cols, rows);
    
    for (let i = 0; i < rows; i++) {
      for (let j = 0; j < cols; j++) {
        result[j][i] = matrix[i][j];
      }
    }
    
    return result;
  }

  /**
   * 高斯消元法求解零空间
   */
  static nullSpace(matrix: Matrix): Matrix {
    if (matrix.length === 0 || matrix[0].length === 0) {
      return [];
    }

    const rows = matrix.length;
    const cols = matrix[0].length;
    
    // 使用分数进行精确计算
    const fractionMatrix = matrix.map(row => 
      row.map(val => new Fraction(val))
    );

    // 高斯-约旦消元
    const augmented = this.gaussJordanElimination(fractionMatrix);
    
    // 找到自由变量
    const pivotCols: number[] = [];
    const freeVars: number[] = [];
    
    for (let i = 0; i < augmented.length; i++) {
      let pivotCol = -1;
      for (let j = 0; j < cols; j++) {
        if (!augmented[i][j].equals(0)) {
          pivotCol = j;
          break;
        }
      }
      if (pivotCol !== -1) {
        pivotCols.push(pivotCol);
      }
    }
    
    for (let j = 0; j < cols; j++) {
      if (!pivotCols.includes(j)) {
        freeVars.push(j);
      }
    }

    if (freeVars.length === 0) {
      return [];
    }

    // 构建零空间基
    const nullSpaceBasis: Matrix = [];
    
    for (const freeVar of freeVars) {
      const solution = new Array(cols).fill(new Fraction(0));
      solution[freeVar] = new Fraction(1);
      
      // 回代求解
      for (let i = augmented.length - 1; i >= 0; i--) {
        let pivotCol = -1;
        for (let j = 0; j < cols; j++) {
          if (!augmented[i][j].equals(0)) {
            pivotCol = j;
            break;
          }
        }
        
        if (pivotCol !== -1 && pivotCol !== freeVar) {
          let sum = new Fraction(0);
          for (let j = pivotCol + 1; j < cols; j++) {
            sum = sum.add(augmented[i][j].mul(solution[j]));
          }
          solution[pivotCol] = sum.neg().div(augmented[i][pivotCol]);
        }
      }
      
      // 转换为整数解
      const integerSolution = this.toIntegerSolution(solution);
      nullSpaceBasis.push(integerSolution);
    }

    return nullSpaceBasis;
  }

  /**
   * 高斯-约旦消元
   */
  private static gaussJordanElimination(matrix: Fraction[][]): Fraction[][] {
    const rows = matrix.length;
    const cols = matrix[0].length;
    const result = matrix.map(row => [...row]);

    let currentRow = 0;
    
    for (let col = 0; col < cols && currentRow < rows; col++) {
      // 找到主元
      let pivotRow = currentRow;
      for (let row = currentRow + 1; row < rows; row++) {
        if (result[row][col].abs().compare(result[pivotRow][col].abs()) > 0) {
          pivotRow = row;
        }
      }
      
      // 如果主元为0，跳过这一列
      if (result[pivotRow][col].equals(0)) {
        continue;
      }
      
      // 交换行
      if (pivotRow !== currentRow) {
        [result[currentRow], result[pivotRow]] = [result[pivotRow], result[currentRow]];
      }
      
      // 归一化主元行
      const pivot = result[currentRow][col];
      for (let j = 0; j < cols; j++) {
        result[currentRow][j] = result[currentRow][j].div(pivot);
      }
      
      // 消元
      for (let i = 0; i < rows; i++) {
        if (i !== currentRow && !result[i][col].equals(0)) {
          const factor = result[i][col];
          for (let j = 0; j < cols; j++) {
            result[i][j] = result[i][j].sub(factor.mul(result[currentRow][j]));
          }
        }
      }
      
      currentRow++;
    }

    return result;
  }

  /**
   * 将分数解转换为整数解
   */
  private static toIntegerSolution(fractionSolution: Fraction[]): number[] {
    // 找到所有分母的最小公倍数
    let lcm = new Fraction(1);
    for (const frac of fractionSolution) {
      if (!frac.equals(0)) {
        lcm = this.lcmFraction(lcm, new Fraction(1, frac.d));
      }
    }

    // 乘以最小公倍数得到整数解
    const integerSolution = fractionSolution.map(frac => {
      const result = frac.mul(lcm);
      return result.n / result.d; // 应该是整数
    });

    // 确保所有系数为正数
    const hasNegative = integerSolution.some(x => x < 0);
    if (hasNegative) {
      return integerSolution.map(x => -x);
    }

    return integerSolution;
  }

  /**
   * 计算两个分数的最小公倍数
   */
  private static lcmFraction(a: Fraction, b: Fraction): Fraction {
    return a.mul(b).div(this.gcdFraction(a, b));
  }

  /**
   * 计算两个分数的最大公约数
   */
  private static gcdFraction(a: Fraction, b: Fraction): Fraction {
    if (b.equals(0)) return a;
    return this.gcdFraction(b, a.mod(b));
  }

  /**
   * 打印矩阵（调试用）
   */
  static printMatrix(matrix: Matrix, title?: string): void {
    if (title) {
      console.log(title);
    }
    for (const row of matrix) {
      console.log(row.map(x => x.toString().padStart(8)).join(' '));
    }
    console.log();
  }
}