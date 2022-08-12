import {BalanceResult, ChemicalCompound, ChemicalEquation, ChemicalNode, ElementCount, Matrix} from './types';
import {MatrixUtils} from './matrix';
import Fraction from 'fraction.js';

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
                return {success: false, equations: [], error: 'No elements found'};
            }

            // 构建系数矩阵
            const matrix = this.buildCoefficientMatrix(compounds, elements, equation);

            // 求解零空间
            const nullSpace = MatrixUtils.nullSpace(matrix);

            if (nullSpace.length === 0) {
                return {success: false, equations: [], error: 'No solution found'};
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
    private buildCoefficientMatrix(compounds: ChemicalCompound[], elements: string[], equation: ChemicalEquation): Matrix {
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
        const reactantCount = equation.reactants.length;

        for (let i = 0; i < matrix.length; i++) {
            for (let j = reactantCount; j < compounds.length; j++) {
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
        // 将所有系数转换为分数，并找到最小的正数分数
        let minPositiveFraction: Fraction | null = null;
        const fractions = solution.map(x => new Fraction(x));

        for (const frac of fractions) {
            if (frac.s === 1) { // 检查是否为正数
                if (minPositiveFraction === null || frac.compare(minPositiveFraction) < 0) {
                    minPositiveFraction = frac;
                }
            }
        }

        if (minPositiveFraction === null) {
            // 如果没有正数，则所有系数都设为1（或根据实际情况处理）
            return solution.map(() => 1);
        }

        // 归一化所有分数，使其最小正数为1
        const normalizedFractions = fractions.map(frac => frac.div(minPositiveFraction!));

        // 找到所有分母的最小公倍数，将所有分数转换为整数
        let lcmDenominator = new Fraction(1);
        for (const frac of normalizedFractions) {
            lcmDenominator = lcmDenominator.lcm(frac.d);
        }

        const integerCoefficients = normalizedFractions.map(frac => frac.mul(lcmDenominator).n);

        // 确保所有系数都是正数，如果零空间解中有负数，则取绝对值
        const finalCoefficients = integerCoefficients.map(x => Math.abs(x));

        // 再次计算最大公约数，进行简化
        const commonDivisor = this.findGCD(finalCoefficients);
        if (commonDivisor > 0) {
            return finalCoefficients.map(x => x / commonDivisor);
        } else {
            return finalCoefficients; // 避免除以零
        }
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